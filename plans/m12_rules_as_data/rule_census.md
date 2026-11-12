# The census of rules: shapes, groupings and connections

**[M12](README.md), P12.1.** Taken 2026-11-12 on the user's instruction of that day:

> Census of the rules shapes - name agnostic, only considering equal names when they reused - so analyze
> only match and assert graphs. count and census all of them. Then connections between rules - first list
> all of possible connections: use same name, use same name in same place, assert->match a relation
> connection, etc. No specific goals right now, want to see analyze results.

and, the same day, *"also utils/rule_census.py can group rule shapes by number of relations, by number of
relations arguments, and so on propose ways to group shapes"*.

**How to re-take it.** Every number and every table between the two `generated` markers below is the output
of one command, run on the tree as `git ls-files` lists it:

```sh
python3 utils/rule_census.py --page plans/m12_rules_as_data/rule_census.md   # the page's tables (runs the engine for one section)
python3 utils/rule_census.py --svg                                            # the census alone, no engine, + the pictures
```

Both write `build/rule_census/` (gitignored), which holds the long tails the page leaves out: `rules.tsv`
(every rule with its shape at every level), `shapes_<level>.tsv` (every shape and its members),
`rule_keys.tsv` and `groups.tsv` (the grouping keys), `connections.tsv`, `degrees.tsv` (a rule's degree
under every kind of connection), `families.json`, `graph_per_file.tsv`, `strata.tsv`, `refused.txt`,
`broken.tsv`, and three DOT pictures: `modules.dot` (the stdlib's modules, an edge where a conclusion of one
unifies with a premise of another; dashed through a generic rule), `rules_arith.dot` and
`rules_algebra.dot`. The numbers quoted in the prose of this page are from the same run (2026-11-12), and
the tables are the authority where the two differ. Edit the script, never the tables.

Contents: [what is counted](#what-is-counted) · [the corpus](#the-corpus-measured) ·
[A. shapes](#a-shapes) · [ways to group shapes](#ways-to-group-shapes) · [B. connections](#b-connections) ·
[what stands out](#what-stands-out) · [what was not computed](#what-was-not-computed)

## What is counted

**A rule** is a `(rule NAME (PARAMS) :match M :assert A)` or an `(hrule …)` written at the top level of a
tracked `.ein` file under `stdlib/`, `examples/` or `tests/`, read **as written**. The script has an
S-expression reader of its own (`;` comments, `#| |#` blocks, strings, the name characters of
[`00_ebnf.md`](../../docs/kernel/ir/03-ein-lang/00_ebnf.md) § 1) and is not the loader:

- **No macro is expanded for a shape and no import is resolved for a shape.** A rule that calls `forall`
  has `forall` in its shape.
- **A rule the loader would refuse is a rule here.** Files under `examples/broken/` are read and counted
  apart and are in no table. Outside that directory the corpus holds witnesses of a refusal (most under
  `examples/ein-bugs/wrong-answers/`); the census reads them as written, and § B's last section says how
  many they are and how many rules they hold.
- Rules written in doc pages and in the Rust test suites are not read.

**A shape** is a rule's `:match` and `:assert` with every name replaced by a token of its kind, equal names
equal tokens, and nothing else of a name kept:

| kind | what | token |
|---|---|---|
| relation | a name written at the head of a form anywhere in the rule, at any depth | `R1`, `R2` … |
| constant | any other name | `c1`, `c2` … |
| parameter | a variable of the rule's parameter list | `?P1`, `?P2` … |
| variable | any other variable | `?a`, `?b` … |

**Structure is kept as written**, being no name: the heads `and`, `or`, `not`, `absent`, `eq`, `neq`,
`false` and `open`, which are the grammar's connectives, guards and verdicts; a macro's name at the head of
a form, `forall` and `unknown` of `std.macro` where the file reaches that module and any macro the file
declares, since a macro call is a connective the program defines; and the kernel's two dunders `__closed__`
and `__symmetric__` wherever they are written, since the engine gives them their meaning. A structure word
in an argument's place is a name: `eq` in `(cmp-of ?x ?y eq)` is the constant it spells. The declarator,
`rule` or `hrule`, is part of the shape, and so is the number of parameters, written `[k]`, at the levels
that tell a parameter from a variable.

**The levels.** Each is a column of `rules.tsv`.

| level | what is abstracted | conjunct order |
|---|---|---|
| the same text (`alpha`) | variables | as written |
| the same rule (`alphau`) | variables | any |
| relations renamed (`relren`) | variables and relations; constants as written | any |
| constants renamed (`conren`) | variables and constants; relations as written | any |
| **(i)** | every name | as written |
| **(ii)** | every name | any |
| **(iii)** | every name, into two kinds alone: whatever heads a form, a name or a variable, is a relation `R`; every other name, variable or parameter is a term `t`. The number of parameters is dropped, so a rule with a relation parameter and the same rule written for one relation are one shape | any |
| **(iii′)** | as (iii), and every term is `_`: which term is which is forgotten, the relations' pattern stays | any |
| **(iv)** | counts alone: declarator, parameters, positive premises, `not`, `absent`, `or`, guards, macro calls, conclusions by kind, the arities of premises and of conclusions, the two nesting depths, variables, constants, relations | — |
| **(iv′)** | five of those: parameters, positive premises, negations, guards, conclusions | — |

*Any order* is a canonical labelling: the children of `and`, of `or` and the two arguments of `eq` and `neq`
are a multiset, in the `:match` and in the `:assert`. Names are split by colour refinement, a name's colour
refined by the literals it occurs in, and the names the refinement leaves together are tried in every order
while that is at most 5 040 orders, the least string winning, which is exact. Past that the script
individualises one name at a time and refines again, which is exact where the tied names are
interchangeable (the hundred relations a closing rule lists) and a heuristic otherwise; it runs that
fallback twice with opposite choices and counts the rules on which the two differ. The count is in the
first section below.

**Scopes, for connections.** *Global*: any two rules of the census, a name equal to a name by spelling
alone. *File*: two rules of one file. *Program*: a rule a file writes and a rule its imports bring. An
import is followed only as far as which module file it names (`std.X` is `stdlib/X.ein`, any other name a
file beside the importing one), a `:symbols` selection is closed over the names a kept rule writes, a
qualified import is taken whole and its renaming ignored, and a module's own imports are followed. That
approximates the loader's selection and is not it. For connections alone a `:match` is read with the
macros the file reaches expanded, so that the premises inside a `forall` are premises.

<!-- generated: utils/rule_census.py --page -->

## The corpus measured

| root | files | files with a rule | `rule` | `hrule` | both | with parameters |
|---|---:|---:|---:|---:|---:|---:|
| `stdlib/` | 12 | 10 | 341 | 0 | 341 | 62 |
| `examples/` | 490 | 362 | 1049 | 118 | 1167 | 270 |
| `tests/` | 130 | 30 | 39 | 6 | 45 | 7 |
| **all** | 632 | 402 | 1429 | 124 | 1553 | 339 |

- tracked `.ein` files under the three roots: 703; in the census: 632; under `examples/broken/` and left out: 66, holding 30 rules the reader can read; files the reader cannot read: 7 (`examples/broken/bare_top_level_atom.ein`, `examples/broken/unclosed_paren.ein`, `examples/ein-bugs/wrong-answers/unicode-space-ideographic.ein`, `examples/ein-bugs/wrong-answers/unicode-space-in-a-name.ein`, `examples/ein-bugs/wrong-answers/unicode-space-line-separator.ein`, `examples/ein-bugs/wrong-answers/unicode-space-nbsp.ein`, `examples/ein-bugs/wrong-answers/unicode-space-next-line.ein`).
- `rule` forms in `stdlib/`: 341 (the check: `cat stdlib/*.ein | grep -c '^(rule '`).
- rules with a clause missing or written twice: other-clause 1.
- `rule`/`hrule` forms with no name or no parameter list, left out: 0.
- distinct rules: 1105 up to a renaming of variables with the order as written (`alpha`), 1104 with conjuncts in any order (`alphau`); the other 449 of the 1553 are a rule some other file also writes.
- canonical labelling: 87 labellings (of 12424) took the individualise-and-refine fallback, and on 0 of them its two runs differ.

### By directory

A rule's group is `stdlib`, `tests/stdlib/<module>`, or the first directory under `examples/`. *Distinct rules* is by `alphau`; *shapes* is level (ii); *its own* counts the shapes no other group has.

| group | files | rules | of them `hrule` | distinct rules | shapes (ii) | its own |
|---|---:|---:|---:|---:|---:|---:|
| `stdlib` | 12 | 341 | 0 | 336 | 272 | 222 |
| `examples (top level)` | 6 | 97 | 4 | 43 | 32 | 1 |
| `examples/arith` | 13 | 15 | 0 | 15 | 15 | 7 |
| `examples/beyond_arith` | 16 | 164 | 4 | 148 | 128 | 117 |
| `examples/binary_arithmetic` | 8 | 80 | 0 | 80 | 60 | 24 |
| `examples/binary_logic` | 20 | 64 | 0 | 62 | 53 | 47 |
| `examples/branching` | 16 | 70 | 6 | 23 | 21 | 8 |
| `examples/domain_elim` | 3 | 25 | 3 | 12 | 10 | 0 |
| `examples/ein-bugs` | 314 | 465 | 71 | 296 | 191 | 143 |
| `examples/features` | 17 | 30 | 2 | 30 | 28 | 10 |
| `examples/lattice` | 20 | 76 | 25 | 48 | 26 | 14 |
| `examples/saturation` | 24 | 24 | 0 | 8 | 8 | 5 |
| `examples/syntax` | 17 | 6 | 2 | 6 | 5 | 5 |
| `examples/word_problems` | 16 | 51 | 1 | 51 | 48 | 45 |
| `tests/region` | 9 | 16 | 6 | 14 | 12 | 0 |
| `tests/stdlib/algebra` | 26 | 3 | 0 | 3 | 1 | 0 |
| `tests/stdlib/arith` | 21 | 2 | 0 | 2 | 2 | 1 |
| `tests/stdlib/bijection` | 6 | 3 | 0 | 3 | 2 | 1 |
| `tests/stdlib/closure` | 3 | 2 | 0 | 1 | 1 | 1 |
| `tests/stdlib/digits` | 2 | 1 | 0 | 1 | 1 | 1 |
| `tests/stdlib/elim` | 4 | 1 | 0 | 1 | 1 | 0 |
| `tests/stdlib/int` | 14 | 2 | 0 | 2 | 2 | 2 |
| `tests/stdlib/macro` | 2 | 3 | 0 | 3 | 3 | 3 |
| `tests/stdlib/num9` | 13 | 4 | 0 | 4 | 4 | 0 |
| `tests/stdlib/rat` | 13 | 2 | 0 | 2 | 2 | 1 |
| `tests/stdlib/slots` | 13 | 5 | 0 | 5 | 2 | 1 |
| `tests/text` | 1 | 1 | 0 | 1 | 1 | 0 |

Groups with files and no rule: `tests/stdlib/typing` (3).

### The stdlib's modules

| module | rules | with parameters | shapes (i) | (ii) | (iii) | (iii′) | (iv) | facts written |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `stdlib/algebra.ein` | 40 | 32 | 38 | 37 | 34 | 23 | 30 | 0 |
| `stdlib/arith.ein` | 89 | 0 | 71 | 71 | 71 | 58 | 59 | 1 |
| `stdlib/bijection.ein` | 8 | 6 | 8 | 8 | 8 | 5 | 5 | 0 |
| `stdlib/closure.ein` | 1 | 0 | 1 | 1 | 1 | 1 | 1 | 0 |
| `stdlib/digits.ein` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1671 |
| `stdlib/elim.ein` | 4 | 4 | 4 | 4 | 4 | 3 | 3 | 0 |
| `stdlib/int.ein` | 71 | 0 | 50 | 50 | 50 | 34 | 34 | 1 |
| `stdlib/macro.ein` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `stdlib/num9.ein` | 47 | 0 | 47 | 47 | 47 | 46 | 39 | 1 |
| `stdlib/rat.ein` | 57 | 0 | 45 | 45 | 45 | 41 | 38 | 1 |
| `stdlib/slots.ein` | 20 | 18 | 20 | 20 | 20 | 13 | 13 | 0 |
| `stdlib/typing.ein` | 4 | 2 | 4 | 4 | 4 | 3 | 3 | 0 |

## A. Shapes

| level | rules | distinct shapes | shapes covering 50 % | 90 % | shapes of one rule | largest shape | over distinct rules: 50 % | 90 % | of one distinct rule |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| the same text: variables renamed | 1553 | 1105 | 329 | 950 | 938 | 23 | 552 | 994 | 1105 |
| the same rule: variables renamed, conjuncts in any order | 1553 | 1104 | 328 | 949 | 936 | 23 | 552 | 994 | 1104 |
| one rule under a renaming of relations | 1553 | 845 | 130 | 690 | 654 | 60 | 293 | 735 | 757 |
| one rule under a renaming of constants | 1553 | 1074 | 298 | 919 | 902 | 23 | 522 | 964 | 1058 |
| (i) every name abstracted, order as written | 1553 | 770 | 101 | 615 | 569 | 60 | 217 | 659 | 652 |
| (ii) every name abstracted, any order | 1553 | 765 | 101 | 610 | 562 | 60 | 213 | 655 | 644 |
| (iii) constants, variables and parameters one kind | 1553 | 708 | 78 | 553 | 516 | 81 | 156 | 598 | 577 |
| (iii′) relations alone; terms forgotten | 1553 | 572 | 49 | 417 | 373 | 102 | 92 | 462 | 409 |
| (iv) the skeleton, every count | 1553 | 627 | 74 | 472 | 399 | 60 | 126 | 517 | 450 |
| (iv′) the skeleton, five counts | 1553 | 123 | 2 | 36 | 49 | 610 | 2 | 41 | 55 |
| `:match` alone, as written | 1553 | 499 | 31 | 344 | 326 | 189 | 42 | 388 | 382 |
| `:match` alone, any order | 1553 | 492 | 30 | 337 | 319 | 189 | 42 | 382 | 371 |
| `:assert` alone, as written | 1553 | 315 | 6 | 160 | 199 | 285 | 10 | 205 | 213 |
| `:assert` alone, any order | 1553 | 306 | 6 | 151 | 197 | 285 | 9 | 196 | 206 |

*Over distinct rules* counts a shape's size in rule texts (`alphau`, 1104 of them) and not in occurrences, so a rule that forty files copy weighs one.

Reading a shape: `[k]` is the number of parameters, `hrule` marks an hrule, `⇒` stands between the `:match` and the `:assert`; `R1` a relation, `c1` a constant, `?a` a variable, `?P1` a parameter, `t1` a term of level (iii), `_` a forgotten term. In the tables' *distinct* column a shape's rules are counted up to `alphau`.

### Level (i): exact shape, order as written

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 60 | 37 | 51 | 1/59/0 | `[0] (R1 ?a) ⇒ (R2 ?a)` | `stdlib/algebra.ein:symmetric-negative-setup`, `examples/arith/num9_overflow.ein:too-big`, `examples/branching/12_typed_blind_solve.ein:derive-functional-negative` |
| 2 | 52 | 15 | 38 | 0/51/1 | `hrule [0] (R1 ?a) ⇒ (R2 ?a)` | `examples/branching/15_lookahead_two_step_on.ein:guess`, `examples/ein-bugs/activator-all-int.ein:check`, `examples/ein-bugs/fork-delta-order.ein:guess` |
| 3 | 24 | 24 | 9 | 16/8/0 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | `stdlib/arith.ein:add-answer`, `stdlib/arith.ein:cmp-answer`, `stdlib/arith.ein:mul-answer` |
| 4 | 23 | 1 | 23 | 1/21/1 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | `stdlib/algebra.ein:functional` |
| 5 | 20 | 15 | 8 | 0/19/1 | `[0] (and (R1 c1) (R1 c2)) ⇒ (false)` | `examples/branching/15_lookahead_two_step_on.ein:pair-fast`, `examples/ein-bugs/wrong-answers/einb-dump-answer.ein:clash`, `examples/features/13_mixed_solution_and_open.ein:mutex` |
| 6 | 18 | 7 | 15 | 0/18/0 | `[0] (and (R1 ?a) (absent (R2 ?a))) ⇒ (R3 ?a)` | `examples/ein-bugs/activator-all-int.ein:flag`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-p`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-q` |
| 7 | 17 | 3 | 9 | 0/17/0 | `[0] (R1 ?a c1) ⇒ (false)` | `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-a`, `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-b`, `examples/ein-bugs/query-goal-free-head-unsat.ein:boom` |
| 8 | 16 | 9 | 16 | 1/15/0 | `[0] (R1 ?a) ⇒ (false)` | `stdlib/arith.ein:division-by-zero-is-false`, `examples/binary_arithmetic/bin.ein:division-by-zero-is-false2`, `examples/ein-bugs/alive-empty-phase1.ein:no-p` |
| 9 | 16 | 1 | 16 | 1/15/0 | `[1] (?P1 ?a ?b) ⇒ (?P1 ?b ?a)` | `stdlib/algebra.ein:symmetric` |
| 10 | 16 | 1 | 16 | 1/14/1 | `[1] (and (?P1 ?a ?b) (?P1 ?b ?c) (neq ?a ?c)) ⇒ (?P1 ?a ?c)` | `stdlib/algebra.ein:transitive` |
| 11 | 15 | 9 | 10 | 0/15/0 | `[0] (and (R1 ?a) (R2 ?a)) ⇒ (R3 ?a)` | `examples/branching/13_lookahead_naf_world.ein:promote`, `examples/branching/14_lookahead_unjudgeable.ein:auto-sign`, `examples/domain_elim/ab.ein:derive-domain-elimination` |
| 12 | 14 | 5 | 11 | 0/12/2 | `hrule [0] (and (R1 ?a c1) (R1 ?b c2)) ⇒ (R2 ?a ?b)` | `examples/beyond_arith/cryptarithm.ein:guess`, `examples/beyond_arith/equation_by_search.ein:guess-digit`, `examples/ein-bugs/wrong-answers/past-limit-lines.ein:seat` |
| 13 | 13 | 2 | 13 | 0/13/0 | `hrule [3] (and (R1 ?a ?P2) (R1 ?b ?P3)) ⇒ (?P1 ?a ?b)` | `examples/branching/09_hrule.ein:guess`, `examples/zebra.ein:guess` |
| 14 | 11 | 9 | 2 | 0/11/0 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (R2 ?a ?b ?c ?d ?e)` | `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-fwd`, `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-bwd`, `examples/ein-bugs/zebra2-hints.ein:derive-disjunctive-prune-fwd` |
| 15 | 11 | 7 | 11 | 0/11/0 | `[0] (R1 ?a) ⇒ (R2 c1)` | `examples/ein-bugs/wrong-answers/alt-cap-shared.ein:x-chain`, `examples/ein-bugs/wrong-answers/forbid-chain-param-absent.ein:mk`, `examples/ein-bugs/wrong-answers/macro-bare-conclusion-under-commitment.ein:mk` |
| 16 | 11 | 5 | 11 | 0/11/0 | `[0] (and (R1 ?a) (R2 ?a)) ⇒ (false)` | `examples/ein-bugs/alt-cap-core-reordered.ein:wide`, `examples/ein-bugs/wrong-answers/alt-cap-shared.ein:clash`, `examples/ein-bugs/wrong-answers/core-search-cut-5.ein:clash` |
| 17 | 11 | 3 | 11 | 0/11/0 | `[1] (R1 ?P1) ⇒ (false)` | `examples/ein-bugs/activator-all-int.ein:alarm`, `examples/ein-bugs/wrong-answers/core-activator-committed.ein:r2`, `examples/ein-bugs/wrong-answers/core-activator-given.ein:r2` |
| 18 | 11 | 1 | 11 | 0/11/0 | `[2] (and (?P1 ?a ?b) (?P1 ?c ?b) (neq ?a ?c)) ⇒ (not (?P2 ?a ?c))` | `examples/branching/01_saturate_only.ein:sibling-exclusive` |
| 19 | 10 | 10 | 2 | 5/5/0 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 (R4 ?a ?b ?c) ?b)` | `stdlib/arith.ein:add-ask-second`, `stdlib/arith.ein:sub-ask-second`, `stdlib/arith.ein:cmp-ask-second` |
| 20 | 10 | 2 | 5 | 0/10/0 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (and (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e) (R4 ?a ?b ?c ?d ?e))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-fwd-fanout`, `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-bwd-fanout` |
| 21 | 10 | 10 | 2 | 5/5/0 | `[0] (R1 ?a ?b) ⇒ (R2 (R3 ?a ?b) ?a)` | `stdlib/arith.ein:add-ask`, `stdlib/arith.ein:sub-ask`, `stdlib/arith.ein:cmp-ask` |
| 22 | 10 | 2 | 9 | 0/10/0 | `[0] (R1 c1 c2) ⇒ (false)` | `examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein:no-soup`, `examples/ein-bugs/wrong-answers/tree-dead.ein:no-stew` |
| 23 | 10 | 10 | 10 | 0/0/10 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | `tests/stdlib/algebra/07_schroder.ein:probe-undecided`, `tests/stdlib/algebra/18_totality_open_world.ein:probe-undecided`, `tests/stdlib/algebra/22_symmetric_negative.ein:probe-undecided` |
| 24 | 10 | 5 | 9 | 0/10/0 | `[1] (R1 ?P1) ⇒ (R2 ?P1)` | `examples/ein-bugs/activator-all-int.ein:watch`, `examples/ein-bugs/wrong-answers/activator-nested-value.ein:act`, `examples/ein-bugs/wrong-answers/forbid-chain-1.ein:q` |
| 25 | 9 | 1 | 9 | 1/7/1 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?b) (neq ?a ?c)) ⇒ (false)` | `stdlib/algebra.ein:injective` |
| 26 | 8 | 7 | 7 | 1/6/1 | `[0] (R1 ?a ?b) ⇒ (R2 ?a)` | `stdlib/arith.ein:let-named`, `examples/binary_arithmetic/bin.ein:let-named2`, `examples/binary_logic/post.ein:set-of` |
| 27 | 8 | 5 | 8 | 0/8/0 | `hrule [0] (R1 ?a c1) ⇒ (R2 ?a)` | `examples/ein-bugs/alive-empty-interlayer.ein:guess`, `examples/ein-bugs/wrong-answers/wild-in-hrule.ein:h`, `examples/features/13_mixed_solution_and_open.ein:pick` |
| 28 | 7 | 7 | 5 | 4/3/0 | `[0] (R1 (R2 ?a) ?b) ⇒ (R3 ?a ?b)` | `stdlib/int.ein:int-simplify-answer`, `stdlib/int.ein:int-neg-answer`, `stdlib/rat.ein:rat-simplify-answer` |
| 29 | 7 | 2 | 6 | 0/7/0 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (and (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via`, `examples/features/09_adjacent_via_same_house.ein:derive-disjunctive-prune` |
| 30 | 7 | 6 | 6 | 0/7/0 | `[0] (R1 c1) ⇒ (R2 c2)` | `examples/ein-bugs/wrong-answers/classes-int-name.ein:a`, `examples/ein-bugs/wrong-answers/range-in-match.ein:r`, `examples/ein-bugs/wrong-answers/str-in-match.ein:t` |

### Level (ii): any order of conjuncts

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 60 | 37 | 51 | 1/59/0 | `[0] (R1 ?a) ⇒ (R2 ?a)` | `stdlib/algebra.ein:symmetric-negative-setup`, `examples/arith/num9_overflow.ein:too-big`, `examples/branching/12_typed_blind_solve.ein:derive-functional-negative` |
| 2 | 52 | 15 | 38 | 0/51/1 | `hrule [0] (R1 ?a) ⇒ (R2 ?a)` | `examples/branching/15_lookahead_two_step_on.ein:guess`, `examples/ein-bugs/activator-all-int.ein:check`, `examples/ein-bugs/fork-delta-order.ein:guess` |
| 3 | 24 | 24 | 9 | 16/8/0 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | `stdlib/arith.ein:add-answer`, `stdlib/arith.ein:cmp-answer`, `stdlib/arith.ein:mul-answer` |
| 4 | 23 | 1 | 23 | 1/21/1 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | `stdlib/algebra.ein:functional` |
| 5 | 20 | 15 | 8 | 0/19/1 | `[0] (and (R1 c1) (R1 c2)) ⇒ (false)` | `examples/branching/15_lookahead_two_step_on.ein:pair-fast`, `examples/ein-bugs/wrong-answers/einb-dump-answer.ein:clash`, `examples/features/13_mixed_solution_and_open.ein:mutex` |
| 6 | 18 | 7 | 15 | 0/18/0 | `[0] (and (R1 ?a) (absent (R2 ?a))) ⇒ (R3 ?a)` | `examples/ein-bugs/activator-all-int.ein:flag`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-p`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-q` |
| 7 | 17 | 3 | 9 | 0/17/0 | `[0] (R1 ?a c1) ⇒ (false)` | `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-a`, `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-b`, `examples/ein-bugs/query-goal-free-head-unsat.ein:boom` |
| 8 | 16 | 9 | 16 | 1/15/0 | `[0] (R1 ?a) ⇒ (false)` | `stdlib/arith.ein:division-by-zero-is-false`, `examples/binary_arithmetic/bin.ein:division-by-zero-is-false2`, `examples/ein-bugs/alive-empty-phase1.ein:no-p` |
| 9 | 16 | 1 | 16 | 1/15/0 | `[1] (?P1 ?a ?b) ⇒ (?P1 ?b ?a)` | `stdlib/algebra.ein:symmetric` |
| 10 | 16 | 1 | 16 | 1/14/1 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?a) (neq ?b ?c)) ⇒ (?P1 ?c ?b)` | `stdlib/algebra.ein:transitive` |
| 11 | 15 | 9 | 10 | 0/15/0 | `[0] (and (R1 ?a) (R2 ?a)) ⇒ (R3 ?a)` | `examples/branching/13_lookahead_naf_world.ein:promote`, `examples/branching/14_lookahead_unjudgeable.ein:auto-sign`, `examples/domain_elim/ab.ein:derive-domain-elimination` |
| 12 | 14 | 5 | 11 | 0/12/2 | `hrule [0] (and (R1 ?a c1) (R1 ?b c2)) ⇒ (R2 ?b ?a)` | `examples/beyond_arith/cryptarithm.ein:guess`, `examples/beyond_arith/equation_by_search.ein:guess-digit`, `examples/ein-bugs/wrong-answers/past-limit-lines.ein:seat` |
| 13 | 13 | 2 | 13 | 0/13/0 | `hrule [3] (and (R1 ?a ?P1) (R1 ?b ?P2)) ⇒ (?P3 ?b ?a)` | `examples/branching/09_hrule.ein:guess`, `examples/zebra.ein:guess` |
| 14 | 11 | 9 | 2 | 0/11/0 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (R2 ?a ?b ?c ?d ?e)` | `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-fwd`, `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-bwd`, `examples/ein-bugs/zebra2-hints.ein:derive-disjunctive-prune-fwd` |
| 15 | 11 | 7 | 11 | 0/11/0 | `[0] (R1 ?a) ⇒ (R2 c1)` | `examples/ein-bugs/wrong-answers/alt-cap-shared.ein:x-chain`, `examples/ein-bugs/wrong-answers/forbid-chain-param-absent.ein:mk`, `examples/ein-bugs/wrong-answers/macro-bare-conclusion-under-commitment.ein:mk` |
| 16 | 11 | 5 | 11 | 0/11/0 | `[0] (and (R1 ?a) (R2 ?a)) ⇒ (false)` | `examples/ein-bugs/alt-cap-core-reordered.ein:wide`, `examples/ein-bugs/wrong-answers/alt-cap-shared.ein:clash`, `examples/ein-bugs/wrong-answers/core-search-cut-5.ein:clash` |
| 17 | 11 | 3 | 11 | 0/11/0 | `[1] (R1 ?P1) ⇒ (false)` | `examples/ein-bugs/activator-all-int.ein:alarm`, `examples/ein-bugs/wrong-answers/core-activator-committed.ein:r2`, `examples/ein-bugs/wrong-answers/core-activator-given.ein:r2` |
| 18 | 11 | 1 | 11 | 0/11/0 | `[2] (and (?P1 ?a ?b) (?P1 ?c ?b) (neq ?a ?c)) ⇒ (not (?P2 ?c ?a))` | `examples/branching/01_saturate_only.ein:sibling-exclusive` |
| 19 | 10 | 10 | 2 | 5/5/0 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 (R4 ?a ?b ?c) ?b)` | `stdlib/arith.ein:add-ask-second`, `stdlib/arith.ein:sub-ask-second`, `stdlib/arith.ein:cmp-ask-second` |
| 20 | 10 | 2 | 5 | 0/10/0 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (and (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e) (R4 ?a ?b ?c ?d ?e))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-fwd-fanout`, `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-bwd-fanout` |
| 21 | 10 | 10 | 2 | 5/5/0 | `[0] (R1 ?a ?b) ⇒ (R2 (R3 ?a ?b) ?a)` | `stdlib/arith.ein:add-ask`, `stdlib/arith.ein:sub-ask`, `stdlib/arith.ein:cmp-ask` |
| 22 | 10 | 2 | 9 | 0/10/0 | `[0] (R1 c1 c2) ⇒ (false)` | `examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein:no-soup`, `examples/ein-bugs/wrong-answers/tree-dead.ein:no-stew` |
| 23 | 10 | 10 | 10 | 0/0/10 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | `tests/stdlib/algebra/07_schroder.ein:probe-undecided`, `tests/stdlib/algebra/18_totality_open_world.ein:probe-undecided`, `tests/stdlib/algebra/22_symmetric_negative.ein:probe-undecided` |
| 24 | 10 | 5 | 9 | 0/10/0 | `[1] (R1 ?P1) ⇒ (R2 ?P1)` | `examples/ein-bugs/activator-all-int.ein:watch`, `examples/ein-bugs/wrong-answers/activator-nested-value.ein:act`, `examples/ein-bugs/wrong-answers/forbid-chain-1.ein:q` |
| 25 | 9 | 1 | 9 | 1/7/1 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?b) (neq ?a ?c)) ⇒ (false)` | `stdlib/algebra.ein:injective` |
| 26 | 8 | 7 | 7 | 1/6/1 | `[0] (R1 ?a ?b) ⇒ (R2 ?a)` | `stdlib/arith.ein:let-named`, `examples/binary_arithmetic/bin.ein:let-named2`, `examples/binary_logic/post.ein:set-of` |
| 27 | 8 | 5 | 8 | 0/8/0 | `hrule [0] (R1 ?a c1) ⇒ (R2 ?a)` | `examples/ein-bugs/alive-empty-interlayer.ein:guess`, `examples/ein-bugs/wrong-answers/wild-in-hrule.ein:h`, `examples/features/13_mixed_solution_and_open.ein:pick` |
| 28 | 7 | 7 | 5 | 4/3/0 | `[0] (R1 (R2 ?a) ?b) ⇒ (R3 ?a ?b)` | `stdlib/int.ein:int-simplify-answer`, `stdlib/int.ein:int-neg-answer`, `stdlib/rat.ein:rat-simplify-answer` |
| 29 | 7 | 2 | 6 | 0/7/0 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (and (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via`, `examples/features/09_adjacent_via_same_house.ein:derive-disjunctive-prune` |
| 30 | 7 | 6 | 6 | 0/7/0 | `[0] (R1 c1) ⇒ (R2 c2)` | `examples/ein-bugs/wrong-answers/classes-int-name.ein:a`, `examples/ein-bugs/wrong-answers/range-in-match.ein:r`, `examples/ein-bugs/wrong-answers/str-in-match.ein:t` |

### Level (iii): constants, variables and parameters one kind

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 81 | 48 | 68 | 2/79/0 | `(R1 t1) ⇒ (R2 t1)` | `stdlib/algebra.ein:imply1`, `stdlib/algebra.ein:symmetric-negative-setup`, `examples/arith/num9_overflow.ein:too-big` |
| 2 | 53 | 16 | 39 | 0/52/1 | `hrule (R1 t1) ⇒ (R2 t1)` | `examples/branching/15_lookahead_two_step_on.ein:guess`, `examples/ein-bugs/activator-all-int.ein:check`, `examples/ein-bugs/fork-delta-order.ein:guess` |
| 3 | 30 | 8 | 27 | 0/28/2 | `hrule (and (R1 t1 t2) (R1 t3 t4)) ⇒ (R2 t3 t1)` | `examples/beyond_arith/cryptarithm.ein:guess`, `examples/beyond_arith/equation_by_search.ein:guess-digit`, `examples/branching/05_mini_zebra.ein:guess` |
| 4 | 28 | 6 | 19 | 1/27/0 | `(R1 t1 t2) ⇒ (false)` | `stdlib/algebra.ein:empty`, `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-a`, `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-b` |
| 5 | 28 | 13 | 28 | 1/27/0 | `(R1 t1) ⇒ (false)` | `stdlib/arith.ein:division-by-zero-is-false`, `examples/binary_arithmetic/bin.ein:division-by-zero-is-false2`, `examples/ein-bugs/activator-all-int.ein:alarm` |
| 6 | 25 | 3 | 25 | 1/23/1 | `(and (R1 t1 t2) (R1 t1 t3) (neq t2 t3)) ⇒ (false)` | `stdlib/algebra.ein:functional`, `examples/binary_logic/circuit.ein:one-value`, `examples/word_problems/readings.ein:one-reading` |
| 7 | 24 | 24 | 9 | 16/8/0 | `(R1 (R2 t1 t2) t3) ⇒ (R3 t1 t2 t3)` | `stdlib/arith.ein:add-answer`, `stdlib/arith.ein:cmp-answer`, `stdlib/arith.ein:mul-answer` |
| 8 | 21 | 16 | 20 | 0/21/0 | `(R1 t1) ⇒ (R2 t2)` | `examples/ein-bugs/wrong-answers/alt-cap-shared.ein:x-chain`, `examples/ein-bugs/wrong-answers/assert-var-stops.ein:r`, `examples/ein-bugs/wrong-answers/assert-var-under-commitment.ein:r` |
| 9 | 20 | 15 | 8 | 0/19/1 | `(and (R1 t1) (R1 t2)) ⇒ (false)` | `examples/branching/15_lookahead_two_step_on.ein:pair-fast`, `examples/ein-bugs/wrong-answers/einb-dump-answer.ein:clash`, `examples/features/13_mixed_solution_and_open.ein:mutex` |
| 10 | 20 | 9 | 17 | 0/20/0 | `(and (R1 t1) (absent (R2 t1))) ⇒ (R3 t1)` | `examples/ein-bugs/activator-all-int.ein:flag`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-p`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-q` |
| 11 | 19 | 3 | 18 | 2/17/0 | `(R1 t1 t2) ⇒ (R1 t2 t1)` | `stdlib/algebra.ein:converse-pair-symmetric`, `stdlib/algebra.ein:symmetric`, `examples/ein-bugs/wrong-answers/uncovered-knows.ein:knows-is-mutual` |
| 12 | 16 | 1 | 16 | 1/14/1 | `(and (R1 t1 t2) (R1 t3 t1) (neq t2 t3)) ⇒ (R1 t3 t2)` | `stdlib/algebra.ein:transitive` |
| 13 | 15 | 9 | 10 | 6/9/0 | `(R1 t1 t2) ⇒ (R2 t1 t2)` | `stdlib/algebra.ein:imply2-fwd`, `stdlib/algebra.ein:join-l`, `stdlib/arith.ein:operand-zero` |
| 14 | 15 | 9 | 10 | 0/15/0 | `(and (R1 t1) (R2 t1)) ⇒ (R3 t1)` | `examples/branching/13_lookahead_naf_world.ein:promote`, `examples/branching/14_lookahead_unjudgeable.ein:auto-sign`, `examples/domain_elim/ab.ein:derive-domain-elimination` |
| 15 | 14 | 2 | 14 | 0/14/0 | `(and (R1 t1 t2) (R1 t3 t2) (neq t1 t3)) ⇒ (not (R2 t3 t1))` | `examples/branching/01_saturate_only.ein:sibling-exclusive`, `examples/saturation/type-exclusivity/colors.ein:type-exclusivity` |
| 16 | 14 | 7 | 14 | 0/12/2 | `hrule (R1 t1 t2) ⇒ (R2 t1)` | `examples/ein-bugs/alive-empty-interlayer.ein:guess`, `examples/ein-bugs/wrong-answers/wild-in-hrule-twin.ein:h`, `examples/ein-bugs/wrong-answers/wild-in-hrule.ein:h` |
| 17 | 12 | 10 | 11 | 1/10/1 | `(R1 t1 t2) ⇒ (R2 t1)` | `stdlib/arith.ein:let-named`, `examples/binary_arithmetic/bin.ein:let-named2`, `examples/binary_logic/post.ein:set-of` |
| 18 | 11 | 9 | 2 | 0/11/0 | `(R1 t1 t2 t3 t4 t5) ⇒ (R2 t1 t2 t3 t4 t5)` | `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-fwd`, `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-bwd`, `examples/ein-bugs/zebra2-hints.ein:derive-disjunctive-prune-fwd` |
| 19 | 11 | 5 | 11 | 0/11/0 | `(and (R1 t1) (R2 t1)) ⇒ (false)` | `examples/ein-bugs/alt-cap-core-reordered.ein:wide`, `examples/ein-bugs/wrong-answers/alt-cap-shared.ein:clash`, `examples/ein-bugs/wrong-answers/core-search-cut-5.ein:clash` |
| 20 | 10 | 10 | 2 | 5/5/0 | `(R1 (R2 t1 t2) t3) ⇒ (R3 (R4 t1 t2 t3) t2)` | `stdlib/arith.ein:add-ask-second`, `stdlib/arith.ein:sub-ask-second`, `stdlib/arith.ein:cmp-ask-second` |
| 21 | 10 | 2 | 5 | 0/10/0 | `(R1 t1 t2 t3 t4 t5) ⇒ (and (R2 t1 t2 t3 t4 t5) (R3 t1 t2 t3 t4 t5) (R4 t1 t2 t3 t4 t5))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-fwd-fanout`, `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-bwd-fanout` |
| 22 | 10 | 10 | 2 | 5/5/0 | `(R1 t1 t2) ⇒ (R2 (R3 t1 t2) t1)` | `stdlib/arith.ein:add-ask`, `stdlib/arith.ein:sub-ask`, `stdlib/arith.ein:cmp-ask` |
| 23 | 10 | 10 | 10 | 0/0/10 | `(unknown (R1 t1 t2)) ⇒ (R2 t1 t2)` | `tests/stdlib/algebra/07_schroder.ein:probe-undecided`, `tests/stdlib/algebra/18_totality_open_world.ein:probe-undecided`, `tests/stdlib/algebra/22_symmetric_negative.ein:probe-undecided` |
| 24 | 9 | 1 | 9 | 1/7/1 | `(and (R1 t1 t2) (R1 t3 t2) (neq t1 t3)) ⇒ (false)` | `stdlib/algebra.ein:injective` |
| 25 | 7 | 7 | 5 | 4/3/0 | `(R1 (R2 t1) t2) ⇒ (R3 t1 t2)` | `stdlib/int.ein:int-simplify-answer`, `stdlib/int.ein:int-neg-answer`, `stdlib/rat.ein:rat-simplify-answer` |
| 26 | 7 | 2 | 6 | 0/7/0 | `(R1 t1 t2 t3 t4 t5) ⇒ (and (R2 t1 t2 t3 t4 t5) (R3 t1 t2 t3 t4 t5))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via`, `examples/features/09_adjacent_via_same_house.ein:derive-disjunctive-prune` |
| 27 | 7 | 1 | 7 | 0/7/0 | `(R1 t1 t2) ⇒ (R2 t3 t2)` | `examples/ein-bugs/zebra2-bad.ein:co-located` |
| 28 | 7 | 1 | 7 | 0/7/0 | `(and (R1 R2 t1 t2) (R3 t3 t4) (R4 t5 t2) (absent (R5 t4 t5))) ⇒ (not (R2 t6 t5))` | `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-bwd` |
| 29 | 7 | 1 | 7 | 0/7/0 | `(and (R1 R2 t1 t2) (R3 t3 t4) (R4 t5 t2) (absent (R5 t5 t4))) ⇒ (not (R2 t6 t5))` | `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-fwd` |
| 30 | 7 | 1 | 7 | 0/7/0 | `(and (R1 t1 t2) (R2 t2 t3) (absent (and (R2 t2 t4) (neq t3 t4)))) ⇒ (R3 t5 t3)` | `examples/ein-bugs/zebra2-bad.ein:adjacent-via-bwd` |

### Level (iii′): the relations' pattern alone

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 102 | 64 | 84 | 2/100/0 | `(R1 _) ⇒ (R2 _)` | `stdlib/algebra.ein:imply1`, `stdlib/algebra.ein:symmetric-negative-setup`, `examples/arith/num9_overflow.ein:too-big` |
| 2 | 55 | 18 | 41 | 0/54/1 | `hrule (R1 _) ⇒ (R2 _)` | `examples/branching/15_lookahead_two_step_on.ein:guess`, `examples/ein-bugs/activator-all-int.ein:check`, `examples/ein-bugs/fork-delta-order.ein:guess` |
| 3 | 36 | 17 | 28 | 9/26/1 | `(R1 _ _) ⇒ (R2 _ _)` | `stdlib/algebra.ein:converse`, `stdlib/algebra.ein:imply2-fwd`, `stdlib/algebra.ein:identity` |
| 4 | 35 | 5 | 25 | 3/30/2 | `(and (R1 _ _) (R1 _ _) (neq _ _)) ⇒ (false)` | `stdlib/algebra.ein:functional`, `stdlib/algebra.ein:injective`, `stdlib/algebra.ein:antisymmetric` |
| 5 | 30 | 8 | 27 | 0/28/2 | `hrule (and (R1 _ _) (R1 _ _)) ⇒ (R2 _ _)` | `examples/beyond_arith/cryptarithm.ein:guess`, `examples/beyond_arith/equation_by_search.ein:guess-digit`, `examples/branching/05_mini_zebra.ein:guess` |
| 6 | 29 | 7 | 19 | 2/27/0 | `(R1 _ _) ⇒ (false)` | `stdlib/algebra.ein:empty`, `stdlib/algebra.ein:irreflexive`, `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-a` |
| 7 | 28 | 13 | 28 | 1/27/0 | `(R1 _) ⇒ (false)` | `stdlib/arith.ein:division-by-zero-is-false`, `examples/binary_arithmetic/bin.ein:division-by-zero-is-false2`, `examples/ein-bugs/activator-all-int.ein:alarm` |
| 8 | 27 | 27 | 10 | 17/10/0 | `(R1 (R2 _ _) _) ⇒ (R3 _ _ _)` | `stdlib/arith.ein:add-answer`, `stdlib/arith.ein:cmp-answer`, `stdlib/arith.ein:mul-answer` |
| 9 | 21 | 5 | 19 | 4/17/0 | `(R1 _ _) ⇒ (R1 _ _)` | `stdlib/algebra.ein:converse-pair-symmetric`, `stdlib/algebra.ein:symmetric`, `stdlib/typing.ein:reflexive-dom` |
| 10 | 21 | 14 | 13 | 0/21/0 | `(and (R1 _) (R2 _)) ⇒ (R3 _)` | `examples/branching/13_lookahead_naf_world.ein:promote`, `examples/branching/14_lookahead_unjudgeable.ein:auto-sign`, `examples/domain_elim/ab.ein:derive-domain-elimination` |
| 11 | 20 | 15 | 8 | 0/19/1 | `(and (R1 _) (R1 _)) ⇒ (false)` | `examples/branching/15_lookahead_two_step_on.ein:pair-fast`, `examples/ein-bugs/wrong-answers/einb-dump-answer.ein:clash`, `examples/features/13_mixed_solution_and_open.ein:mutex` |
| 12 | 20 | 9 | 17 | 0/20/0 | `(and (R1 _) (absent (R2 _))) ⇒ (R3 _)` | `examples/ein-bugs/activator-all-int.ein:flag`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-p`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-q` |
| 13 | 18 | 13 | 17 | 2/15/1 | `(R1 _ _) ⇒ (R2 _)` | `stdlib/algebra.ein:self-converse-is-symmetric`, `stdlib/arith.ein:let-named`, `examples/binary_arithmetic/bin.ein:let-named2` |
| 14 | 17 | 8 | 17 | 0/17/0 | `(and (R1 _) (R2 _)) ⇒ (false)` | `examples/ein-bugs/alt-cap-core-reordered.ein:wide`, `examples/ein-bugs/complete-records-stale.ein:kill-p`, `examples/ein-bugs/fork-delta-order.ein:clash` |
| 15 | 16 | 1 | 16 | 1/14/1 | `(and (R1 _ _) (R1 _ _) (neq _ _)) ⇒ (R1 _ _)` | `stdlib/algebra.ein:transitive` |
| 16 | 14 | 2 | 7 | 0/14/0 | `(and (R1 R2 _ _) (R3 _ _) (R4 _ _) (absent (R5 _ _))) ⇒ (not (R2 _ _))` | `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-fwd`, `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-bwd` |
| 17 | 14 | 2 | 14 | 0/14/0 | `(and (R1 _ _) (R1 _ _) (neq _ _)) ⇒ (not (R2 _ _))` | `examples/branching/01_saturate_only.ein:sibling-exclusive`, `examples/saturation/type-exclusivity/colors.ein:type-exclusivity` |
| 18 | 14 | 2 | 7 | 0/14/0 | `(and (R1 _ _) (R2 _ _) (absent (and (R2 _ _) (neq _ _)))) ⇒ (R3 _ _)` | `examples/ein-bugs/zebra2-bad.ein:adjacent-via-fwd`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-bwd` |
| 19 | 14 | 7 | 14 | 0/12/2 | `hrule (R1 _ _) ⇒ (R2 _)` | `examples/ein-bugs/alive-empty-interlayer.ein:guess`, `examples/ein-bugs/wrong-answers/wild-in-hrule-twin.ein:h`, `examples/ein-bugs/wrong-answers/wild-in-hrule.ein:h` |
| 20 | 12 | 10 | 2 | 0/12/0 | `(R1 _ _ _ _ _) ⇒ (R2 _ _ _ _ _)` | `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-fwd`, `examples/ein-bugs/zebra2-hints.ein:derive-adjacent-via-bwd`, `examples/ein-bugs/zebra2-hints.ein:derive-disjunctive-prune-fwd` |
| 21 | 12 | 12 | 8 | 3/6/3 | `(R1 _ _ _) ⇒ (R2 _ _)` | `stdlib/arith.ein:turn-end`, `stdlib/arith.ein:top-first-end`, `stdlib/rat.ein:rat-gcd-end` |
| 22 | 12 | 2 | 6 | 0/12/0 | `(and (R1 R2 _ _) (R3 _ _) (absent (and (R3 _ _) (R4 _ _)))) ⇒ (not (R2 _ _))` | `examples/ein-bugs/zebra2-bad.ein:adjacent-via-endpoint-fwd`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-endpoint-bwd` |
| 23 | 12 | 2 | 6 | 0/12/0 | `(and (R1 _ _) (absent (and (R1 _ _) (neq _ _))) (not (R2 _ _))) ⇒ (not (R3 _ _))` | `examples/ein-bugs/zebra2-bad.ein:adjacent-via-fwd-negative`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-bwd-negative` |
| 24 | 11 | 8 | 7 | 4/7/0 | `(R1 (R2 _) _) ⇒ (R3 _ _)` | `stdlib/int.ein:int-simplify-answer`, `stdlib/int.ein:int-neg-answer`, `stdlib/rat.ein:rat-simplify-answer` |
| 25 | 11 | 4 | 6 | 2/9/0 | `(and (R1 _ _) (R2 R1 _ _) (R3 _ _) (neq _ _)) ⇒ (not (R1 _ _))` | `stdlib/bijection.ein:functional-negative`, `stdlib/bijection.ein:injective-negative`, `examples/branching/12_typed_blind_solve.ein:functional-negative` |
| 26 | 10 | 10 | 2 | 5/5/0 | `(R1 (R2 _ _) _) ⇒ (R3 (R4 _ _ _) _)` | `stdlib/arith.ein:add-ask-second`, `stdlib/arith.ein:sub-ask-second`, `stdlib/arith.ein:cmp-ask-second` |
| 27 | 10 | 2 | 5 | 0/10/0 | `(R1 _ _ _ _ _) ⇒ (and (R2 _ _ _ _ _) (R3 _ _ _ _ _) (R4 _ _ _ _ _))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-fwd-fanout`, `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-bwd-fanout` |
| 28 | 10 | 10 | 2 | 5/5/0 | `(R1 _ _) ⇒ (R2 (R3 _ _) _)` | `stdlib/arith.ein:add-ask`, `stdlib/arith.ein:sub-ask`, `stdlib/arith.ein:cmp-ask` |
| 29 | 10 | 6 | 5 | 4/6/0 | `(and (R1 _ _) (absent (R2 _ _))) ⇒ (false)` | `stdlib/bijection.ein:typecheck-arg-0`, `stdlib/bijection.ein:typecheck-arg-1`, `examples/branching/12_typed_blind_solve.ein:typecheck-arg-0` |
| 30 | 10 | 10 | 10 | 0/0/10 | `(unknown (R1 _ _)) ⇒ (R2 _ _)` | `tests/stdlib/algebra/07_schroder.ein:probe-undecided`, `tests/stdlib/algebra/18_totality_open_world.ein:probe-undecided`, `tests/stdlib/algebra/22_symmetric_negative.ein:probe-undecided` |

### Level (iv): the skeleton

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 60 | 37 | 51 | 1/59/0 | `rule params=0 premises=1 not=0 absent=0 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=1 conclusion-arities=1 depth=1/1 vars=1 consts=0 relations=2` | `stdlib/algebra.ein:symmetric-negative-setup`, `examples/arith/num9_overflow.ein:too-big`, `examples/branching/12_typed_blind_solve.ein:derive-functional-negative` |
| 2 | 52 | 15 | 38 | 0/51/1 | `hrule params=0 premises=1 not=0 absent=0 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=1 conclusion-arities=1 depth=1/1 vars=1 consts=0 relations=2` | `examples/branching/15_lookahead_two_step_on.ein:guess`, `examples/ein-bugs/activator-all-int.ein:check`, `examples/ein-bugs/fork-delta-order.ein:guess` |
| 3 | 32 | 2 | 23 | 2/28/2 | `rule params=1 premises=2 not=0 absent=0 or=0 guards=1 macros=0 conclusions=0 neg-conclusions=0 false=1 open=0 checks=0 premise-arities=2,2 conclusion-arities= depth=2/1 vars=3 consts=0 relations=0` | `stdlib/algebra.ein:functional`, `stdlib/algebra.ein:injective` |
| 4 | 26 | 26 | 9 | 17/9/0 | `rule params=0 premises=1 not=0 absent=0 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=2 conclusion-arities=3 depth=2/1 vars=3 consts=0 relations=3` | `stdlib/arith.ein:add-answer`, `stdlib/arith.ein:cmp-answer`, `stdlib/arith.ein:mul-answer` |
| 5 | 20 | 15 | 8 | 0/19/1 | `rule params=0 premises=2 not=0 absent=0 or=0 guards=0 macros=0 conclusions=0 neg-conclusions=0 false=1 open=0 checks=0 premise-arities=1,1 conclusion-arities= depth=2/1 vars=0 consts=2 relations=1` | `examples/branching/15_lookahead_two_step_on.ein:pair-fast`, `examples/ein-bugs/wrong-answers/einb-dump-answer.ein:clash`, `examples/features/13_mixed_solution_and_open.ein:mutex` |
| 6 | 18 | 7 | 15 | 0/18/0 | `rule params=0 premises=2 not=0 absent=1 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=1,1 conclusion-arities=1 depth=3/1 vars=1 consts=0 relations=3` | `examples/ein-bugs/activator-all-int.ein:flag`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-p`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-q` |
| 7 | 18 | 3 | 17 | 3/15/0 | `rule params=1 premises=1 not=0 absent=0 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=2 conclusion-arities=2 depth=1/1 vars=2 consts=0 relations=0` | `stdlib/algebra.ein:symmetric`, `stdlib/typing.ein:reflexive-dom`, `stdlib/typing.ein:reflexive-cod` |
| 8 | 17 | 3 | 9 | 0/17/0 | `rule params=0 premises=1 not=0 absent=0 or=0 guards=0 macros=0 conclusions=0 neg-conclusions=0 false=1 open=0 checks=0 premise-arities=2 conclusion-arities= depth=1/1 vars=1 consts=1 relations=1` | `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-a`, `examples/ein-bugs/alive-set-fresh-name-declared.ein:no-b`, `examples/ein-bugs/query-goal-free-head-unsat.ein:boom` |
| 9 | 16 | 9 | 16 | 1/15/0 | `rule params=0 premises=1 not=0 absent=0 or=0 guards=0 macros=0 conclusions=0 neg-conclusions=0 false=1 open=0 checks=0 premise-arities=1 conclusion-arities= depth=1/1 vars=1 consts=0 relations=1` | `stdlib/arith.ein:division-by-zero-is-false`, `examples/binary_arithmetic/bin.ein:division-by-zero-is-false2`, `examples/ein-bugs/alive-empty-phase1.ein:no-p` |
| 10 | 16 | 1 | 16 | 1/14/1 | `rule params=1 premises=2 not=0 absent=0 or=0 guards=1 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=2,2 conclusion-arities=2 depth=2/1 vars=3 consts=0 relations=0` | `stdlib/algebra.ein:transitive` |
| 11 | 15 | 9 | 10 | 0/15/0 | `rule params=0 premises=2 not=0 absent=0 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=1,1 conclusion-arities=1 depth=2/1 vars=1 consts=0 relations=3` | `examples/branching/13_lookahead_naf_world.ein:promote`, `examples/branching/14_lookahead_unjudgeable.ein:auto-sign`, `examples/domain_elim/ab.ein:derive-domain-elimination` |
| 12 | 14 | 5 | 11 | 0/12/2 | `hrule params=0 premises=2 not=0 absent=0 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=2,2 conclusion-arities=2 depth=2/1 vars=2 consts=2 relations=2` | `examples/beyond_arith/cryptarithm.ein:guess`, `examples/beyond_arith/equation_by_search.ein:guess-digit`, `examples/ein-bugs/wrong-answers/past-limit-lines.ein:seat` |
| 13 | 14 | 2 | 7 | 0/14/0 | `rule params=5 premises=3 not=0 absent=1 or=0 guards=1 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=2,2,2 conclusion-arities=2 depth=4/1 vars=3 consts=0 relations=0` | `examples/ein-bugs/zebra2-bad.ein:adjacent-via-fwd`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-bwd` |
| 14 | 14 | 2 | 7 | 0/14/0 | `rule params=5 premises=4 not=0 absent=1 or=0 guards=0 macros=0 conclusions=0 neg-conclusions=1 false=0 open=0 checks=0 premise-arities=2,2,2,3 conclusion-arities=2 depth=3/2 vars=4 consts=0 relations=2` | `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-fwd`, `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-bwd` |
| 15 | 13 | 2 | 13 | 0/13/0 | `hrule params=3 premises=2 not=0 absent=0 or=0 guards=0 macros=0 conclusions=1 neg-conclusions=0 false=0 open=0 checks=0 premise-arities=2,2 conclusion-arities=2 depth=2/1 vars=2 consts=0 relations=1` | `examples/branching/09_hrule.ein:guess`, `examples/zebra.ein:guess` |

### Level (iv′): five counts

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 610 | 521 | 176 | 207/388/15 | `rule params=0 premises=1 negations=0 guards=0 conclusions=1` | `stdlib/algebra.ein:symmetric-is-self-converse`, `stdlib/algebra.ein:self-converse-is-symmetric`, `stdlib/algebra.ein:converse-pair-symmetric` |
| 2 | 169 | 138 | 68 | 35/131/3 | `rule params=0 premises=2 negations=0 guards=0 conclusions=1` | `stdlib/arith.ein:add-column`, `stdlib/arith.ein:add-column-first`, `stdlib/arith.ein:add-column-second` |
| 3 | 74 | 34 | 57 | 0/73/1 | `hrule params=0 premises=1 negations=0 guards=0 conclusions=1` | `examples/branching/15_lookahead_two_step_on.ein:guess`, `examples/ein-bugs/activator-all-int.ein:check`, `examples/ein-bugs/alive-empty-interlayer.ein:guess` |
| 4 | 55 | 23 | 45 | 5/50/0 | `rule params=1 premises=1 negations=0 guards=0 conclusions=1` | `stdlib/algebra.ein:empty`, `stdlib/algebra.ein:irreflexive`, `stdlib/algebra.ein:symmetric` |
| 5 | 52 | 5 | 30 | 4/45/3 | `rule params=1 premises=2 negations=0 guards=1 conclusions=1` | `stdlib/algebra.ein:functional`, `stdlib/algebra.ein:injective`, `stdlib/algebra.ein:antisymmetric` |
| 6 | 47 | 39 | 30 | 2/43/2 | `rule params=0 premises=3 negations=0 guards=0 conclusions=1` | `stdlib/arith.ein:operand-name`, `stdlib/num9.ein:mul9-overflows-shift`, `examples/arith/factorial.ein:next-product` |
| 7 | 31 | 14 | 28 | 0/31/0 | `rule params=0 premises=2 negations=1 guards=0 conclusions=1` | `examples/branching/13_lookahead_naf_world.ein:self-block`, `examples/ein-bugs/activator-all-int.ein:flag`, `examples/ein-bugs/naf-upward-closure.ein:bad` |
| 8 | 26 | 4 | 7 | 0/26/0 | `rule params=5 premises=4 negations=1 guards=0 conclusions=1` | `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-fwd`, `examples/ein-bugs/zebra2-bad.ein:disjunctive-prune-bwd`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-endpoint-fwd` |
| 9 | 24 | 22 | 11 | 0/24/0 | `rule params=0 premises=1 negations=0 guards=1 conclusions=1` | `examples/beyond_arith/casting_out_nines.ein:nines-differ`, `examples/beyond_arith/laws.ein:add-commutes-not`, `examples/beyond_arith/laws.ein:mul-commutes-not` |
| 10 | 23 | 14 | 17 | 4/19/0 | `rule params=0 premises=1 negations=0 guards=0 conclusions=2` | `stdlib/algebra.ein:derive-join`, `stdlib/arith.ein:sub-start`, `stdlib/int.ein:int-norm-walks` |
| 11 | 22 | 9 | 17 | 0/20/2 | `rule params=0 premises=3 negations=1 guards=0 conclusions=1` | `examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein:owes-food`, `examples/lattice/10_fifo_gap.ein:r2`, `examples/lattice/synth/local_not_global.ein:owe-1` |
| 12 | 17 | 8 | 14 | 0/15/2 | `hrule params=0 premises=2 negations=0 guards=0 conclusions=1` | `examples/beyond_arith/cryptarithm.ein:guess`, `examples/beyond_arith/equation_by_search.ein:guess-digit`, `examples/ein-bugs/wrong-answers/hrule-eq.ein:h` |
| 13 | 17 | 16 | 16 | 0/7/10 | `rule params=0 premises=0 negations=0 guards=0 conclusions=1` | `examples/ein-bugs/wrong-answers/assert-var-never-fires.ein:r`, `examples/ein-bugs/wrong-answers/bare-premise.ein:a`, `examples/ein-bugs/wrong-answers/bare-premise.ein:c` |
| 14 | 15 | 14 | 7 | 2/13/0 | `rule params=0 premises=4 negations=0 guards=0 conclusions=1` | `stdlib/algebra.ein:compose-contravariant`, `stdlib/algebra.ein:join-converse`, `examples/beyond_arith/cryptarithm.ein:add-column-alone` |
| 15 | 15 | 3 | 13 | 0/15/0 | `rule params=2 premises=2 negations=0 guards=1 conclusions=1` | `examples/branching/01_saturate_only.ein:sibling-exclusive`, `examples/ein-bugs/wrong-answers/alt-flag-refuted-by.ein:occupied`, `examples/ein-bugs/wrong-answers/alt-flag-refuted-by.ein:elsewhere` |

### The `:match` alone (any order)

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 189 | 111 | 136 | 9/179/1 | `(R1 ?a)` | `stdlib/algebra.ein:symmetric-is-self-converse`, `stdlib/algebra.ein:bijective-properties`, `stdlib/algebra.ein:symmetric-negative-setup` |
| 2 | 67 | 53 | 41 | 19/46/2 | `(R1 ?a ?b)` | `stdlib/algebra.ein:converse-pair-symmetric`, `stdlib/arith.ein:let-named`, `stdlib/arith.ein:add-ask` |
| 3 | 51 | 51 | 10 | 27/24/0 | `(R1 (R2 ?a ?b) ?c)` | `stdlib/arith.ein:add-ask-second`, `stdlib/arith.ein:add-answer`, `stdlib/arith.ein:sub-ask-second` |
| 4 | 39 | 21 | 25 | 2/37/0 | `(R1 ?a c1)` | `stdlib/arith.ein:operand-zero`, `stdlib/arith.ein:strip-none`, `examples/binary_arithmetic/bin.ein:operand-zero2` |
| 5 | 35 | 20 | 26 | 1/34/0 | `(and (R1 ?a) (R2 ?a))` | `stdlib/closure.ein:infer-closure`, `examples/beyond_arith/laws.ein:divisors`, `examples/branching/13_lookahead_naf_world.ein:promote` |
| 6 | 33 | 18 | 10 | 4/29/0 | `(R1 ?a ?b ?c ?d ?e)` | `stdlib/int.ein:int-add-plus`, `stdlib/int.ein:int-mul-first`, `stdlib/int.ein:int-cmp-left` |
| 7 | 29 | 7 | 21 | 10/19/0 | `(?P1 ?a ?b)` | `stdlib/algebra.ein:converse`, `stdlib/algebra.ein:imply2-fwd`, `stdlib/algebra.ein:join-l` |
| 8 | 29 | 29 | 6 | 14/15/0 | `(R1 (R2 ?a ?b ?c) ?d)` | `stdlib/arith.ein:add-start`, `stdlib/arith.ein:sub-start`, `stdlib/arith.ein:cmp-start` |
| 9 | 23 | 1 | 23 | 1/21/1 | `(and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c))` | `stdlib/algebra.ein:functional` |
| 10 | 22 | 16 | 8 | 0/21/1 | `(and (R1 c1) (R1 c2))` | `examples/branching/15_lookahead_two_step_on.ein:pair-fast`, `examples/branching/15_lookahead_two_step_on.ein:pair-slow-1`, `examples/ein-bugs/wrong-answers/einb-dump-answer.ein:clash` |
| 11 | 21 | 8 | 14 | 0/21/0 | `(R1 ?P1)` | `examples/ein-bugs/activator-all-int.ein:watch`, `examples/ein-bugs/activator-all-int.ein:alarm`, `examples/ein-bugs/wrong-answers/activator-nested-value.ein:act` |
| 12 | 21 | 9 | 18 | 0/21/0 | `(and (R1 ?a) (absent (R2 ?a)))` | `examples/ein-bugs/activator-all-int.ein:flag`, `examples/ein-bugs/naf-upward-closure.ein:bad`, `examples/ein-bugs/wrong-answers/f07-fpoff.ein:derive-p` |
| 13 | 20 | 2 | 20 | 1/18/1 | `(and (?P1 ?a ?b) (?P1 ?c ?b) (neq ?a ?c))` | `stdlib/algebra.ein:injective`, `examples/branching/01_saturate_only.ein:sibling-exclusive` |
| 14 | 19 | 19 | 19 | 4/15/0 | `()` | `stdlib/arith.ein:arith`, `stdlib/int.ein:int-arith`, `stdlib/num9.ein:arith9` |
| 15 | 16 | 13 | 11 | 1/13/2 | `(R1 c1)` | `stdlib/num9.ein:to9-zero`, `examples/ein-bugs/alive-empty-interlayer.ein:taint`, `examples/ein-bugs/alive-empty-interlayer.ein:kill` |
| 16 | 16 | 1 | 16 | 1/14/1 | `(and (?P1 ?a ?b) (?P1 ?c ?a) (neq ?b ?c))` | `stdlib/algebra.ein:transitive` |
| 17 | 16 | 3 | 16 | 0/16/0 | `(and (R1 ?a ?P1) (R1 ?b ?P2))` | `examples/branching/05_mini_zebra.ein:guess`, `examples/branching/09_hrule.ein:guess`, `examples/zebra.ein:guess` |
| 18 | 14 | 11 | 7 | 6/8/0 | `(R1 (R2 ?a) ?b)` | `stdlib/int.ein:int-norm-plus`, `stdlib/int.ein:int-norm-minus`, `stdlib/int.ein:int-simplify-answer` |
| 19 | 14 | 5 | 11 | 0/12/2 | `(and (R1 ?a c1) (R1 ?b c2))` | `examples/beyond_arith/cryptarithm.ein:guess`, `examples/beyond_arith/equation_by_search.ein:guess-digit`, `examples/ein-bugs/wrong-answers/past-limit-lines.ein:seat` |
| 20 | 13 | 7 | 12 | 2/11/0 | `(R1 ?a ?b ?c ?d)` | `stdlib/rat.ein:rat-norm-numerator`, `stdlib/slots.ein:slot-spatial-setup`, `examples/beyond_arith/fibonacci.ein:fib-here` |

### The `:assert` alone (any order)

| # | rules | distinct | files | stdlib/examples/tests | shape | examples |
|---:|---:|---:|---:|---|---|---|
| 1 | 285 | 189 | 181 | 10/268/7 | `(R1 ?a)` | `stdlib/algebra.ein:self-converse-is-symmetric`, `stdlib/algebra.ein:symmetric-negative-setup`, `stdlib/arith.ein:let-named` |
| 2 | 210 | 105 | 129 | 19/185/6 | `(false)` | `stdlib/algebra.ein:converse-illtyped-dom`, `stdlib/algebra.ein:converse-illtyped-ran`, `stdlib/algebra.ein:empty` |
| 3 | 124 | 98 | 63 | 15/104/5 | `(R1 ?a ?b)` | `stdlib/algebra.ein:converse-pair-symmetric`, `stdlib/arith.ein:operand-name`, `stdlib/arith.ein:add-end` |
| 4 | 79 | 26 | 42 | 22/56/1 | `(?P1 ?a ?b)` | `stdlib/algebra.ein:converse`, `stdlib/algebra.ein:imply2-fwd`, `stdlib/algebra.ein:compose` |
| 5 | 53 | 51 | 22 | 22/31/0 | `(R1 ?a ?b ?c)` | `stdlib/algebra.ein:compose-contravariant`, `stdlib/algebra.ein:join-converse`, `stdlib/arith.ein:add-answer` |
| 6 | 47 | 33 | 40 | 0/45/2 | `(R1 c1)` | `examples/beyond_arith/column_lemma.ein:walk-end`, `examples/ein-bugs/alive-empty-interlayer.ein:taint`, `examples/ein-bugs/complete-records-stale.ein:guess-q` |
| 7 | 45 | 8 | 8 | 0/45/0 | `(not (?P1 ?P2 ?a))` | `examples/ein-bugs/zebra2-bad.ein:co-located-negative`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-fwd-negative`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-bwd-negative` |
| 8 | 41 | 20 | 24 | 14/27/0 | `(not (?P1 ?a ?b))` | `stdlib/algebra.ein:symmetric-negative`, `stdlib/algebra.ein:compose-negative-s`, `stdlib/algebra.ein:compose-negative-r` |
| 9 | 24 | 20 | 16 | 2/18/4 | `(R1 ?a c1)` | `stdlib/arith.ein:operand-zero`, `stdlib/arith.ein:strip-none`, `examples/arith/int.ein:colder` |
| 10 | 21 | 3 | 7 | 0/21/0 | `(?P1 ?P2 ?a)` | `examples/ein-bugs/zebra2-bad.ein:co-located`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-fwd`, `examples/ein-bugs/zebra2-bad.ein:adjacent-via-bwd` |
| 11 | 19 | 18 | 10 | 10/9/0 | `(R1 ?a ?b ?c ?d)` | `stdlib/arith.ein:cmp-column`, `stdlib/arith.ein:mul-first-row`, `stdlib/arith.ein:mul-next` |
| 12 | 19 | 17 | 19 | 0/8/11 | `(R1 c1 c2)` | `examples/arith/mul.ein:fence`, `examples/ein-bugs/wrong-answers/oblig-derived-activator-declared.ein:arm`, `examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein:special` |
| 13 | 19 | 7 | 16 | 0/17/2 | `(open c1)` | `examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein:owes-food`, `examples/lattice/06_friends_tables.ein:group-seated-owed`, `examples/lattice/synth/local_not_global.ein:owe-1` |
| 14 | 16 | 14 | 5 | 0/16/0 | `(R1 ?a ?b ?c ?d ?e)` | `examples/binary_logic/props_rules.ein:column-of`, `examples/binary_logic/simplify.ein:normal-kept`, `examples/binary_logic/simplify.ein:normal-swapped` |
| 15 | 13 | 8 | 12 | 0/13/0 | `(R1 ?P1)` | `examples/ein-bugs/activator-all-int.ein:watch`, `examples/ein-bugs/wrong-answers/activator-nested-value.ein:act`, `examples/ein-bugs/wrong-answers/empty-match-activator.ein:r` |
| 16 | 10 | 3 | 10 | 1/7/2 | `(?P1 ?a)` | `stdlib/algebra.ein:imply1`, `examples/ein-bugs/wrong-answers/eq-arity-conclusion.ein:r`, `examples/lattice/01_subset_pruned.ein:guess` |
| 17 | 10 | 10 | 2 | 5/5/0 | `(R1 (R2 ?a ?b ?c) ?b)` | `stdlib/arith.ein:add-ask-second`, `stdlib/arith.ein:sub-ask-second`, `stdlib/arith.ein:cmp-ask-second` |
| 18 | 10 | 10 | 2 | 5/5/0 | `(R1 (R2 ?a ?b) ?a)` | `stdlib/arith.ein:add-ask`, `stdlib/arith.ein:sub-ask`, `stdlib/arith.ein:cmp-ask` |
| 19 | 10 | 2 | 5 | 0/10/0 | `(and (R1 ?a ?b ?c ?d ?e) (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e))` | `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-fwd-fanout`, `examples/ein-bugs/zebra2-bad.ein:derive-adjacent-via-bwd-fanout` |
| 20 | 9 | 8 | 3 | 0/9/0 | `(R1 c1 ?a ?b)` | `examples/beyond_arith/equation_by_search.ein:col-units`, `examples/beyond_arith/laws.ein:add-commutes`, `examples/beyond_arith/laws.ein:add-commutes-not` |

### The skeleton's numbers

Each over the rules of the census: least / median / mean / 90th percentile / greatest.

|  | every rule | distinct rules |
|---|---|---|
| parameters | 0 / 0 / 0.5 / 2 / 5 | 0 / 0 / 0.3 / 1 / 5 |
| positive premises | 0 / 1 / 2.2 / 3 / 257 | 0 / 1 / 2.4 / 3 / 257 |
| `not` and `absent` | 0 / 0 / 0.1 / 1 / 3 | 0 / 0 / 0.1 / 0 / 3 |
| `eq`/`neq` guards | 0 / 0 / 0.1 / 1 / 3 | 0 / 0 / 0.1 / 0 / 3 |
| conclusions | 0 / 1 / 1.6 / 1 / 162 | 0 / 1 / 1.7 / 1 / 162 |
| variables that are no parameter | 0 / 2 / 3.9 / 6 / 257 | 0 / 3 / 4.7 / 7 / 257 |
| constants | 0 / 0 / 1.2 / 2 / 161 | 0 / 0 / 1.6 / 2 / 161 |
| relation names | 0 / 2 / 2.5 / 5 / 11 | 0 / 3 / 2.8 / 5 / 11 |
| widest atom (arguments) | 0 / 2 / 2.5 / 5 / 8 | 0 / 2 / 2.8 / 5 / 8 |
| nesting depth of the `:match` | 0 / 2 / 2.0 / 3 / 11 | 0 / 2 / 2.1 / 3 / 11 |
| nesting depth of the `:assert` | 0 / 1 / 1.4 / 2 / 11 | 0 / 1 / 1.5 / 3 / 11 |

| rules with | rules | distinct | in `stdlib/` |
|---|---:|---:|---:|
| no premise (`:match ()`) | 19 | 19 | 4 |
| a `:match` of one positive premise and nothing else | 810 | 605 | 227 |
| an `absent`, a `not` or a macro call in the `:match` | 242 | 147 | 35 |
| an `or` in the `:match` | 10 | 9 | 0 |
| a guard | 185 | 88 | 24 |
| concludes `(false)` | 210 | 105 | 19 |
| concludes `(open …)` | 28 | 16 | 4 |
| a nested fact (depth of 3 or more in either clause, the connective counted) | 452 | 359 | 165 |
| a variable at the head of a premise or a conclusion | 299 | 112 | 62 |
| no constant at all | 920 | 564 | 184 |

### Families

A *family* is a set of two or more distinct rules (`alphau`) that one coarser level cannot tell apart.

| families whose members | families | distinct rules in one | largest |
|---|---:|---:|---:|
| differ by constants alone (relations and structure equal) | 16 | 46 | 12 |
| differ by relation names alone (constants and structure equal) | 88 | 347 | 37 |
| differ by names alone: one shape at level (ii) | 121 | 460 | 37 |
| one shape at level (iii) | 131 | 527 | 48 |
| one shape at level (iii′) | 163 | 695 | 64 |

**By what is written in a slot.** Two rules of one shape *as written* have their names slot for slot. Families whose members are one text but for constants (relations, variables' pattern and order equal), by the number of constant slots that vary:

| constant slots that vary | families | distinct rules in one | largest |
|---|---:|---:|---:|
| 1 | 4 | 8 | 2 |
| 2 | 10 | 34 | 12 |

The largest that vary in one constant:

| rules | the values of the slot | where | members |
|---:|---|---|---|
| 2 | `Soup`, `Stew` | 2 files | `no-soup`, `no-stew` |
| 2 | `eq`, `gt` | `examples/beyond_arith/primes.ein` | `trial-on-eq`, `trial-on-gt` |
| 2 | `A`, `B` | `examples/ein-bugs/alive-set-fresh-name-declared.ein` | `no-a`, `no-b` |
| 2 | `Water`, `Tea` | `examples/syntax/query-full.ein` | `h1`, `h2` |

The largest that vary in two:

| rules | the values of the slot | where | members |
|---:|---|---|---|
| 12 | `A`, `B`, `C`, `D`, `E`, `F`; `C`, `B`, `D`, `F`, `G` | 3 files | `dead-1`, `dead-1`, `dead-3`, `dead-4` … |
| 4 | `Bob`, `Cat`, `S1`; `Dog`, `Rat`, `S2` | 4 files | `probe-undecided`, `probe-undecided`, `probe-undecided`, `probe-undecided` |
| 3 | `A`, `B`, `C`; `K1`, `K2` | 2 files | `owe-1`, `owe-2`, `owe-2` |
| 3 | `b`, `1`, `_`; `t`, `u`, `w` | 2 files | `t`, `u`, `w` |
| 2 | `F`, `G`; `B`, `D` | `examples/lattice/synth/three_components.ein` | `dead-3`, `dead-7` |
| 2 | `A`, `a`; `B`, `b` | 2 files | `pair-fast`, `clash` |

Families whose members are one text but for relation names (constants equal), by the number of relation slots that vary; a family that varies in one slot is one rule with a relation for a parameter, written out:

| relation slots that vary | families | distinct rules in one | largest |
|---|---:|---:|---:|
| 1 | 25 | 65 | 9 |
| 2 | 26 | 131 | 37 |
| 3 | 23 | 103 | 24 |
| 4 or more | 14 | 48 | 10 |

The largest that vary in one relation:

| rules | the values of the slot | where | members |
|---:|---|---|---|
| 9 | `division-by-zero2`, `p`, `q`, `u`, `x`, `y`, `bad`, `a`, `division-by-zero` | 9 files | `division-by-zero-is-false2`, `no-p`, `narrow`, `k1` … |
| 5 | `rat-add`, `rat-sub`, `rat-cmp`, `rat-mul`, `rat-div` | `stdlib/rat.ein` | `rat-add-ask`, `rat-sub-ask`, `rat-cmp-ask`, `rat-mul-ask` … |
| 4 | `int-add`, `int-sub`, `int-mul`, `int-cmp` | `stdlib/int.ein` | `int-add-ask`, `int-sub-ask`, `int-mul-ask`, `int-cmp-ask` |
| 3 | `int-div-euclid`, `int-div-trunc`, `int-div-floor` | `stdlib/int.ein` | `int-div-euclid-ask`, `int-div-trunc-ask`, `int-div-floor-ask` |
| 3 | `h`, `paid`, `seen` | 3 files | `r`, `deny`, `deny` |
| 3 | `item`, `seed`, `p` | 3 files | `alarm`, `r2`, `r2` |
| 2 | `knows`, `converse` | 2 files | `knows-is-mutual`, `converse-pair-symmetric` |
| 2 | `p`, `s` | 2 files | `r`, `r` |
| 2 | `int-simplify`, `int-neg` | `stdlib/int.ein` | `int-simplify-ask`, `int-neg-ask` |
| 2 | `rat-simplify`, `rat-neg` | `stdlib/rat.ein` | `rat-simplify-ask`, `rat-neg-ask` |
| 2 | `p`, `copy` | 2 files | `r`, `start` |
| 2 | `together`, `apart` | `examples/word_problems/words.ein` | `together-plan`, `apart-plan` |

The largest that vary in two:

| rules | the values of the slot | where | members |
|---:|---|---|---|
| 37 | `overflow`, `functional`, `injective`, `w1`, `paid`, `g`, `p`, `start`, `item` …; `too-big`, `functional-negative`, `injective-negative`, `w3`, `admitted`, `u`, `v`, `w`, `r2` … | 33 files | `too-big`, `derive-functional-negative`, `derive-injective-negative`, `mk-w3` … |
| 15 | `cand`, `item`, `sw`, `seed`, `slot`, `p`, `thing`; `p`, `checked`, `on`, `r2`, `p2`, `tag`, `oorz`, `orz`, `q` … | 13 files | `guess`, `check`, `guess`, `g` … |
| 9 | `adjacent-via`, `adjacent-via-fwd`, `adjacent-via-bwd`; `adjacent-via-fwd`, `adjacent-via-bwd`, `disjunctive-prune`, `adjacent-via-fwd-negative`, `adjacent-via-bwd-negative`, `disjunctive-prune-fwd`, `disjunctive-prune-bwd`, `adjacent-via-endpoint-fwd`, `adjacent-via-endpoint-bwd` | 2 files | `derive-adjacent-via-fwd`, `derive-adjacent-via-bwd`, `derive-disjunctive-prune-fwd`, `derive-adjacent-via-fwd-negative` … |
| 7 | `let2`, `member`, `inside`, `likes`, `let`, `lives`; `named2`, `set`, `incomplete`, `bad`, `glad`, `named`, `housed` | 6 files | `let-named2`, `set-of`, `incomplete-r`, `taint` … |
| 5 | `turn2`, `top-first2`, `to-dec-go`, `turn`, `top-first`; `done2`, `top-first-is2`, `dec-of`, `done`, `top-first-is` | 3 files | `turn-end2`, `top-first-end2`, `to-dec-answer`, `turn-end` … |
| 5 | `un-walk`, `add-walk2`, `sub-walk2`, `add-walk`, `sub-walk`; `un-strip`, `strip2`, `strip` | 3 files | `un-end`, `add-end2`, `sub-end2`, `add-end` … |
| 5 | `w3`, `x`, `A`, `p`, `seated`; `w2`, `z`, `B`, `q`, `paid` | 5 files | `wide`, `clash`, `clash`, `p-and-q-clash` … |
| 5 | `item`, `p`, `seed`; `alarm`, `hit`, `r2`, `r3`, `x` | 5 files | `watch`, `act`, `q`, `r2` … |

Inside one file: 117 families at level (ii) (287 distinct rules), 235 at level (iii′) (559 distinct rules). The largest at level (iii′), where a family's members may differ in which variable sits where:

| rules | file | shapes (ii) among them | members |
|---:|---|---:|---|
| 9 | `stdlib/arith.ein` | 9 | `div-try-1`, `div-try-2`, `div-try-3`, `div-try-4`, `div-try-5`, `div-try-6` … |
| 8 | `stdlib/int.ein` | 5 | `int-div-trunc-pp`, `int-div-trunc-pn`, `int-div-trunc-np`, `int-div-trunc-nn`, `int-div-floor-pp`, `int-div-floor-nn` … |
| 8 | `stdlib/rat.ein` | 2 | `rat-norm-reduce`, `rat-cross-end`, `rat-add-answer`, `rat-sub-answer`, `rat-cmp-answer`, `rat-mul-reduce` … |
| 8 | `examples/zebra2-hints.ein` | 1 | `derive-adjacent-via-fwd`, `derive-adjacent-via-bwd`, `derive-adjacent-via-fwd-negative`, `derive-adjacent-via-bwd-negative`, `derive-disjunctive-prune-fwd`, `derive-disjunctive-prune-bwd` … |
| 8 | `examples/lattice/synth/three_components.ein` | 1 | `dead-1`, `dead-2`, `dead-4`, `dead-5`, `dead-6`, `dead-8` … |
| 6 | `stdlib/int.ein` | 1 | `int-add-norm`, `int-add-answer`, `int-sub-answer`, `int-mul-norm`, `int-mul-answer`, `int-cmp-answer` |
| 6 | `examples/lattice/synth/one_model_per_component.ein` | 1 | `dead-1`, `dead-2`, `dead-3`, `dead-4`, `dead-5`, `dead-6` |
| 5 | `examples/binary_arithmetic/bin.ein` | 1 | `add-ask-second2`, `sub-ask-second2`, `cmp-ask-second2`, `mul-ask-second2`, `div-ask-second2` |
| 5 | `stdlib/arith.ein` | 1 | `add-ask-second`, `sub-ask-second`, `cmp-ask-second`, `mul-ask-second`, `div-ask-second` |
| 5 | `stdlib/rat.ein` | 1 | `rat-add-ask`, `rat-sub-ask`, `rat-cmp-ask`, `rat-mul-ask`, `rat-div-ask` |
| 5 | `examples/binary_arithmetic/bin.ein` | 1 | `add-ask2`, `sub-ask2`, `cmp-ask2`, `mul-ask2`, `div-ask2` |
| 5 | `stdlib/arith.ein` | 1 | `add-ask`, `sub-ask`, `cmp-ask`, `mul-ask`, `div-ask` |
| 5 | `examples/binary_logic/post.ein` | 1 | `inside-zero`, `inside-one`, `inside-monotone`, `inside-affine`, `inside-self-dual` |
| 4 | `stdlib/int.ein` | 3 | `int-div-euclid-np-more`, `int-div-euclid-nn-more`, `int-div-floor-pn-more`, `int-div-floor-np-more` |
| 4 | `stdlib/int.ein` | 4 | `int-div-euclid-pp`, `int-div-euclid-pn`, `int-div-euclid-np-exact`, `int-div-euclid-nn-exact` |

## Ways to group shapes

A grouping key is a function of a rule that looks at no name's spelling, so it is a function of the level-(ii) shape and groups shapes. The catalogue:

| key | definition | groups | largest, in rules | largest, in shapes (ii) |
|---|---|---:|---:|---:|
| **relations** | the number of distinct heads of forms in the two clauses, a name or a variable, at any depth; structure words are not counted | 9 | 527 | 220 |
| **arity profile** | the multiset of the numbers of arguments of every form headed by a relation, at any depth, both clauses | 247 | 237 | 49 |
| **arguments in all** | the sum of that multiset | 14 | 276 | 213 |
| **widest atom** | its greatest member | 10 | 495 | 188 |
| **premises × conclusions** | positive premises (at any depth of `and`/`or`) by conclusions (a fact, a stored negation, `(false)`, `(open …)` and a concluded guard each one) | 16 | 898 | 407 |
| **variables** | distinct variables, parameters among them | 11 | 410 | 132 |
| **variables a premise** | that number over the number of positive premises | 6 | 490 | 213 |
| **constants** | distinct names that head no form | 6 | 920 | 356 |
| **parameters** | the length of the parameter list | 6 | 1214 | 644 |
| **negation profile** | how many `not`, how many `absent`, how many macro calls the `:match` holds | 10 | 1311 | 644 |
| **guard profile** | how many `eq`, how many `neq` the `:match` holds | 6 | 1402 | 695 |
| **nesting** | the depth of the deepest positive premise and of the deepest conclusion, a flat atom 1 | 19 | 751 | 291 |
| **builds or takes apart** | the two depths compared: a conclusion deeper than every premise *builds* (a constructor), the reverse *takes apart* | 6 | 751 | 291 |
| **join structure** | the positive premises as a hypergraph, a premise the set of its variables: no premise, one, a product (some two share nothing, the graph disconnected), cyclic (the GYO reduction does not empty it), else a pair, a star (one variable in every premise), a chain (each premise shares with at most two) or a tree; guards and negations are not edges, and a parameter is no shared variable, being bound before any premise is matched | 8 | 947 | 429 |
| **join components** | the connected components of that graph: 2 or more is a cross product | 5 | 1322 | 646 |
| **feeds itself** | whether a conclusion's relation is the relation of a positive premise of the same rule | 4 | 1110 | 564 |
| **conclusion variables** | where the variables of the positive conclusions come from: none (ground), parameters alone, one premise (and for a rule of one premise and one conclusion, flat: a copy, a permutation, a projection, a constant put in), several premises, or a variable no positive premise binds | 12 | 354 | 229 |
| **schema** | the level-(iii) shape is exactly one of the schemas the script lists (`SCHEMAS`: inclusion, converse, symmetry, transitivity, composition, functional, injective, …), a relation a name or a parameter alike | 20 | 1141 | 696 |
| **rule or hrule** | the declarator | 2 | 1429 | 733 |

### By relations

| relations | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 0 | 6 | 4 | 4 | 0 | `[1] (eq c1 ?P1) ⇒ (false)` | 3 |
| 1 | 204 | 94 | 60 | 14 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 2 | 527 | 338 | 195 | 42 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 3 | 415 | 320 | 220 | 112 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| 4 | 218 | 193 | 145 | 85 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 (R4 ?a ?b ?c) ?b)` | 10 |
| 5 | 135 | 109 | 99 | 61 | `[5] (and (?P1 ?P2 ?a) (R1 ?P3 ?b ?c) (R2 ?d ?c) (absent (?P4 ?a ?d))) ⇒ (not (?P3 ?P5 ?d))` | 7 |
| 6 | 36 | 36 | 33 | 23 | `[0] (and (R1 ?a) (R2 ?a) (R3 ?a) (R4 ?a) (R5 ?a)) ⇒ (R6 ?a)` | 3 |
| 7 | 5 | 5 | 5 | 1 | `[0] (and (R1 ?a ?b c1) (R2 (R3 ?c ?d ?e ?f)) (R4 (R5 ?d ?e ?f) ?b) (R4 (R5 ?c ?e ?f) ?a)) ⇒ (and (R6 ?c ?d …` | 1 |
| 8 or more | 7 | 5 | 4 | 3 | `[0] (and (R1 ?a) (R2 ?a) (R3 ?a) (R4 ?a) (R5 ?a) (R6 ?a) (R7 ?a) (R8 ?a)) ⇒ (R9 ?a)` | 4 |

### By arity profile

| arity profile | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 1 1 | 237 | 138 | 49 | 2 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 2 2 | 133 | 58 | 37 | 22 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 2 2 2 | 96 | 44 | 25 | 13 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?a) (neq ?b ?c)) ⇒ (?P1 ?c ?b)` | 16 |
| 1 1 1 | 66 | 44 | 25 | 3 | `[0] (and (R1 ?a) (absent (R2 ?a))) ⇒ (R3 ?a)` | 18 |
| 2 2 2 2 | 57 | 30 | 24 | 17 | `[5] (and (?P1 ?P2 ?a) (?P3 ?a ?b) (absent (and (?P3 ?a ?c) (neq ?b ?c)))) ⇒ (?P4 ?P5 ?b)` | 7 |
| 1 | 56 | 39 | 26 | 1 | `[0] (R1 ?a) ⇒ (false)` | 16 |
| 1 2 | 52 | 35 | 21 | 3 | `[0] (R1 ?a ?b) ⇒ (R2 ?a)` | 8 |
| 2 2 3 | 37 | 37 | 12 | 17 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| 1 1 2 | 33 | 18 | 16 | 1 | `[0] (and (R1 ?a) (absent (and (R2 ?b) (R3 c1 ?b)))) ⇒ (open R2)` | 7 |
| 2 2 2 2 3 | 31 | 9 | 9 | 2 | `[5] (and (?P1 ?P2 ?a) (R1 ?P3 ?b ?c) (R2 ?d ?c) (absent (?P4 ?a ?d))) ⇒ (not (?P3 ?P5 ?d))` | 7 |
| 2 2 2 3 | 31 | 22 | 13 | 11 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 (R4 ?a ?b ?c) ?b)` | 10 |
| 2 | 30 | 8 | 5 | 2 | `[0] (R1 ?a c1) ⇒ (false)` | 17 |
| 1 1 1 1 | 22 | 17 | 14 | 0 | `[0] (and (R1 ?a) (R2 ?a) (R3 ?a)) ⇒ (R4 ?a)` | 5 |
| 1 2 2 | 22 | 14 | 8 | 4 | `[0] (R1 (R2 ?a) ?b) ⇒ (R3 ?a ?b)` | 7 |

The 233 smaller groups are in `groups.tsv`.

### By arguments in all

| arguments in all | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 0 | 8 | 5 | 5 | 0 | `[1] (eq c1 ?P1) ⇒ (false)` | 3 |
| 1 | 57 | 40 | 27 | 1 | `[0] (R1 ?a) ⇒ (false)` | 16 |
| 2 | 269 | 147 | 55 | 4 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 3 | 118 | 79 | 46 | 6 | `[0] (and (R1 ?a) (absent (R2 ?a))) ⇒ (R3 ?a)` | 18 |
| 4 | 193 | 98 | 71 | 23 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 5 | 60 | 45 | 29 | 10 | `[0] (R1 (R2 ?a) ?b) ⇒ (R3 ?a ?b)` | 7 |
| 6 | 134 | 80 | 50 | 19 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?a) (neq ?b ?c)) ⇒ (?P1 ?c ?b)` | 16 |
| 7 | 56 | 56 | 26 | 25 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| 8 | 93 | 62 | 45 | 26 | `[5] (and (?P1 ?P2 ?a) (?P3 ?a ?b) (absent (and (?P3 ?a ?c) (neq ?b ?c)))) ⇒ (?P4 ?P5 ?b)` | 7 |
| 9 | 67 | 56 | 32 | 30 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 (R4 ?a ?b ?c) ?b)` | 10 |
| 10 | 71 | 68 | 45 | 19 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (R2 ?a ?b ?c ?d ?e)` | 11 |
| 11 | 59 | 36 | 34 | 9 | `[5] (and (?P1 ?P2 ?a) (R1 ?P3 ?b ?c) (R2 ?d ?c) (absent (?P4 ?a ?d))) ⇒ (not (?P3 ?P5 ?d))` | 7 |
| 12 to 23 | 276 | 243 | 213 | 106 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (and (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e) (R4 ?a ?b ?c ?d ?e))` | 10 |
| 24 or more | 92 | 89 | 87 | 63 | `[0] (R1 (R2 (R3 ?a ?b) c1 c2 ?c) ?d (R4 ?e ?f)) ⇒ (R5 (R6 (R3 ?a ?b) c2 ?c (R4 ?e ?f)) ?d (R4 c3 c4) c5 c3)` | 2 |

### By widest atom

| widest atom | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 0 | 8 | 5 | 5 | 0 | `[1] (eq c1 ?P1) ⇒ (false)` | 3 |
| 1 | 410 | 259 | 131 | 11 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 2 | 495 | 273 | 188 | 92 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 3 | 230 | 190 | 131 | 63 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| 4 | 140 | 131 | 103 | 49 | `[0] (R1 (R2 ?a ?b) ?c ?d) ⇒ (R3 ?a ?b ?c ?d)` | 5 |
| 5 | 173 | 151 | 113 | 46 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (R2 ?a ?b ?c ?d ?e)` | 11 |
| 6 | 22 | 22 | 21 | 9 | `[0] (R1 (R2 ?a ?b ?c ?d ?e ?f) ?g) ⇒ (R3 (R4 ?a ?b ?c ?d ?e) ?f ?g c1 c2)` | 2 |
| 7 | 10 | 10 | 10 | 10 | `[0] (R1 (R2 (R3 ?a ?b) ?c ?d ?e) ?f ?g ?h) ⇒ (R4 (R3 ?a ?b) ?c ?d ?e ?f ?g ?h)` | 1 |
| 8 | 4 | 4 | 4 | 4 | `[0] (R1 (R2 ?a ?b ?c ?d ?e ?f ?g) ?h) ⇒ (R3 (R4 ?a ?b ?c ?d ?e ?f ?g ?h) ?c c1 c2 c3)` | 1 |
| 9 or more | 61 | 59 | 59 | 57 | `[0] (R1 (R2 c1 c1 c1 c1 c1 c1 c2 c3 c4) (R2 c1 c1 c1 c1 c1 c1 c5 c6 c7) ?a) ⇒ (R3 ?a (R2 c1 c1 c1 c1 c1 c1 …` | 2 |

### By premises × conclusions

| premises × conclusions | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 1 × 1 | 898 | 670 | 407 | 227 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 2 × 1 | 348 | 204 | 153 | 56 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 3 × 1 | 104 | 71 | 65 | 12 | `[5] (and (?P1 ?P2 ?a) (R1 ?P3 ?b ?c) (R2 ?d ?c) (absent (?P4 ?a ?d))) ⇒ (not (?P3 ?P5 ?d))` | 7 |
| 0 × 1 | 39 | 30 | 20 | 1 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | 10 |
| 6 or more × 1 | 33 | 29 | 28 | 19 | `[0] (and (R1 ?a) (R2 ?a) (R3 ?a) (R4 ?a) (R5 ?a) (R6 ?a) (R7 ?a) (R8 ?a)) ⇒ (R9 ?a)` | 4 |
| 4 × 1 | 29 | 26 | 26 | 8 | `[2] (and (?P1 ?a ?b) (?P1 ?c ?d) (R1 ?a ?P2) (R2 ?c ?a) (absent (and (?P1 ?a ?e) (neq ?b ?e)))) ⇒ (R2 ?d ?b)` | 3 |
| 1 × 2 | 24 | 15 | 13 | 4 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (and (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e))` | 7 |
| 5 × 1 | 24 | 19 | 17 | 4 | `[0] (and (R1 ?a) (R2 ?a) (R3 ?a) (R4 ?a) (R5 ?a)) ⇒ (R6 ?a)` | 3 |
| 0 × 4 or more | 13 | 13 | 11 | 4 | `[0] () ⇒ (and (R1 c1) (R1 c2) (R1 c3) (R1 c4) (R1 c5) (R1 c6) (__closed__ c7) (__closed__ c8) (__closed__ c…` | 2 |
| 1 × 3 | 12 | 3 | 2 | 0 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (and (R2 ?a ?b ?c ?d ?e) (R3 ?a ?b ?c ?d ?e) (R4 ?a ?b ?c ?d ?e))` | 10 |
| 1 × 4 or more | 12 | 7 | 6 | 3 | `[0] (R1 ?a) ⇒ (and (R2 ?a) (R3 ?a) (R4 ?a) (R5 ?a))` | 6 |
| 2 × 2 | 7 | 7 | 7 | 1 | `[0] (and (R1 ?a ?b ?c ?d ?e ?f) (R2 (R3 ?a ?c ?f))) ⇒ (and (R2 (R3 ?a ?c ?e)) (R4 (R3 ?a ?c ?f) (R3 ?a ?c ?…` | 1 |
| 4 × 2 | 4 | 4 | 4 | 0 | `[0] (and (R1 ?a ?b ?c ?d) (R2 ?e ?a) (R2 (R3 ?f) ?b) (R4 (R5 ?e ?f))) ⇒ (and (R2 (R5 ?e ?f) ?c) (R2 (R6 ?e …` | 1 |
| 3 × 2 | 3 | 3 | 3 | 1 | `[0] (and (R1 ?a ?b ?c) (R2 ?a) (R3 ?d)) ⇒ (and (R4 ?a ?d ?c) (R5 ?a ?d ?b))` | 1 |

The 2 smaller groups are in `groups.tsv`.

### By variables

| variables | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 0 | 97 | 75 | 41 | 6 | `[0] (and (R1 c1) (R1 c2)) ⇒ (false)` | 20 |
| 1 | 410 | 257 | 132 | 17 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 2 | 226 | 174 | 112 | 40 | `hrule [0] (and (R1 ?a c1) (R1 ?b c2)) ⇒ (R2 ?b ?a)` | 14 |
| 3 | 174 | 145 | 91 | 54 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| 4 | 215 | 143 | 113 | 62 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 5 | 156 | 97 | 79 | 39 | `hrule [3] (and (R1 ?a ?P1) (R1 ?b ?P2)) ⇒ (?P3 ?b ?a)` | 13 |
| 6 | 78 | 67 | 62 | 32 | `[1] (and (?P1 ?a ?b) (R1 ?P1 ?c ?d) (R2 ?e ?c) (neq ?a ?e)) ⇒ (not (?P1 ?e ?b))` | 5 |
| 7 | 56 | 52 | 44 | 25 | `[0] (and (R1 ?a c1 ?b ?c ?d) (R2 ?e (R3 ?f ?a) c2 ?b ?g)) ⇒ (R2 ?e ?f c2 ?d (R4 ?c ?g))` | 4 |
| 8 | 56 | 32 | 32 | 19 | `[5] (and (?P1 ?P2 ?a) (?P3 ?a ?b) (absent (and (?P3 ?a ?c) (neq ?b ?c)))) ⇒ (?P4 ?P5 ?b)` | 7 |
| 9 | 46 | 23 | 20 | 12 | `[5] (and (?P1 ?P2 ?a) (R1 ?P3 ?b ?c) (R2 ?d ?c) (absent (?P4 ?a ?d))) ⇒ (not (?P3 ?P5 ?d))` | 7 |
| 10 or more | 39 | 39 | 39 | 35 | `[0] (R1 (R2 (R2 (R2 (R2 (R2 (R2 (R2 (R2 (R2 (R2 ?a ?b) ?c) ?d) ?e) ?f) ?g) ?h) ?i) ?j) ?k)) ⇒ (R3 (R2 (R2 (…` | 1 |

### By variables a premise

| variables a premise | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 1 to under 2 | 490 | 328 | 213 | 32 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 2 to under 3 | 345 | 217 | 155 | 63 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 3 to under 5 | 374 | 302 | 213 | 150 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| 5 or more | 154 | 120 | 101 | 87 | `[0] (R1 ?a ?b ?c ?d ?e) ⇒ (R2 ?a ?b ?c ?d ?e)` | 11 |
| no premise | 52 | 43 | 31 | 5 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | 10 |
| under 1 | 138 | 94 | 52 | 4 | `[0] (and (R1 c1) (R1 c2)) ⇒ (false)` | 20 |

### By constants

| constants | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 0 | 920 | 564 | 356 | 184 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 1 | 270 | 223 | 165 | 43 | `[0] (R1 ?a c1) ⇒ (false)` | 17 |
| 2 | 260 | 222 | 158 | 70 | `[0] (and (R1 c1) (R1 c2)) ⇒ (false)` | 20 |
| 3 | 62 | 61 | 55 | 35 | `[0] (R1 c1 c2 ?a) ⇒ (R2 ?a c3)` | 3 |
| 4 | 13 | 10 | 9 | 0 | `[0] (R1 (R2 ?a ?b) c1) ⇒ (R3 (R4 ?b) ?a (R5 c2 c3) c4 c2)` | 4 |
| 5 or more | 28 | 24 | 22 | 9 | `[0] () ⇒ (and (R1 c1) (R1 c2) (R1 c3) (R1 c4) (R1 c5) (R1 c6) (__closed__ c7) (__closed__ c8) (__closed__ c…` | 2 |

### By parameters

| parameters | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 0 | 1214 | 972 | 644 | 279 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 1 | 157 | 58 | 50 | 12 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| 2 | 64 | 25 | 23 | 13 | `[2] (and (?P1 ?a ?b) (?P1 ?c ?b) (neq ?a ?c)) ⇒ (not (?P2 ?c ?a))` | 11 |
| 3 | 32 | 18 | 17 | 17 | `hrule [3] (and (R1 ?a ?P1) (R1 ?b ?P2)) ⇒ (?P3 ?b ?a)` | 13 |
| 4 | 26 | 15 | 15 | 13 | `[4] (?P1 ?P2 ?a) ⇒ (?P3 ?P4 ?a)` | 7 |
| 5 | 60 | 16 | 16 | 7 | `[5] (and (?P1 ?P2 ?a) (?P3 ?a ?b) (absent (and (?P3 ?a ?c) (neq ?b ?c)))) ⇒ (?P4 ?P5 ?b)` | 7 |

### By negation profile

| negation profile | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| not 0, absent 0 | 1311 | 957 | 644 | 306 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| not 0, absent 1 | 139 | 69 | 56 | 18 | `[0] (and (R1 ?a) (absent (R2 ?a))) ⇒ (R3 ?a)` | 18 |
| not 0, absent 0, macro 1 | 59 | 50 | 37 | 10 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | 10 |
| not 1, absent 0 | 17 | 12 | 12 | 4 | `[4] (not (?P1 ?P2 ?a)) ⇒ (not (?P3 ?P4 ?a))` | 6 |
| not 1, absent 1 | 14 | 4 | 4 | 2 | `[5] (and (?P1 ?a ?b) (absent (and (?P1 ?a ?c) (neq ?b ?c))) (not (?P2 ?P3 ?b))) ⇒ (not (?P4 ?P5 ?a))` | 6 |
| not 0, absent 2 | 5 | 5 | 5 | 1 | `[0] (and (R1 ?a ?b) (absent (R1 ?b ?c)) (absent (R1 ?a ?c))) ⇒ (R1 ?b ?a)` | 1 |
| not 0, absent 1, macro 1 | 3 | 3 | 3 | 0 | `[0] (and (R1 ?a c1) (absent (said (and (R2 ?a))))) ⇒ (R3 ?a)` | 1 |
| not 2, absent 0 | 3 | 2 | 2 | 0 | `[0] (and (not (R1 c1)) (not (R1 c2))) ⇒ (false)` | 2 |
| not 1, absent 0, macro 1 | 1 | 1 | 1 | 0 | `[0] (and (R1 ?a ?b) (eq ?a ?a) (forall ?c (R1 ?b ?c) (R1 ?c ?b)) (not (R1 ?b ?a))) ⇒ (and (R1 ?b ?a) (not (…` | 1 |
| not 1, absent 2 | 1 | 1 | 1 | 0 | `[0] (and (absent (R1 c1)) (absent (R1 c2)) (not (R1 c3))) ⇒ (false)` | 1 |

### By guard profile

| guard profile | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| eq 0, neq 0 | 1402 | 1026 | 695 | 321 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| eq 0, neq 1 | 134 | 64 | 58 | 20 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| eq 1, neq 0 | 13 | 10 | 8 | 0 | `[0] (and (R1 ?a) (eq c1 ?a)) ⇒ (R2 ?a)` | 3 |
| eq 0, neq 2 | 2 | 2 | 2 | 0 | `[0] (and (R1 ?a ?b ?c ?d ?e ?f) (R2 (R3 ?g ?c ?e) ?h) (R4 (R3 ?g ?c ?f)) (neq ?b ?g) (neq ?g ?a)) ⇒ (R2 (R3…` | 1 |
| eq 0, neq 3 | 1 | 1 | 1 | 0 | `[0] (and (R1 ?a) (R2 ?b) (R2 ?c) (R2 ?d) (neq ?b ?c) (neq ?b ?d) (neq ?c ?d) (not (R3 ?a ?b)) (not (R3 ?a ?…` | 1 |
| eq 1, neq 1 | 1 | 1 | 1 | 0 | `[0] (and (eq c1 ?a) (neq c1 ?b)) ⇒ (R1 ?a ?b)` | 1 |

### By nesting

| nesting | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| premises 1, conclusions 1 | 751 | 444 | 291 | 72 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| premises 1, conclusions 0 | 253 | 136 | 93 | 23 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| premises 2, conclusions 2 | 168 | 163 | 121 | 66 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 (R4 ?a ?b ?c) ?b)` | 10 |
| premises 2, conclusions 1 | 132 | 126 | 67 | 49 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| premises 1, conclusions 2 | 59 | 56 | 45 | 18 | `[0] (R1 ?a ?b) ⇒ (R2 (R3 ?a ?b) ?a)` | 10 |
| premises 0, conclusions 1 | 43 | 37 | 25 | 5 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | 10 |
| premises 3, conclusions 2 | 33 | 33 | 27 | 30 | `[0] (R1 (R2 (R3 ?a ?b) ?c ?d) (R4 ?e ?f)) ⇒ (R5 (R3 ?a ?b) ?c ?d ?e ?f)` | 3 |
| premises 2, conclusions 3 | 31 | 31 | 30 | 23 | `[0] (R1 ?a ?b (R2 ?c ?d)) ⇒ (R3 (R4 ?a (R2 ?c ?d)) ?b (R2 ?c ?d))` | 2 |
| premises 3, conclusions 3 | 25 | 25 | 24 | 22 | `[0] (R1 (R2 (R3 ?a ?b) c1 c2 ?c) ?d (R4 ?e ?f)) ⇒ (R5 (R6 (R3 ?a ?b) c2 ?c (R4 ?e ?f)) ?d (R4 c3 c4) c5 c3)` | 2 |
| premises 1, conclusions 4 or more | 10 | 10 | 3 | 10 | `[0] (R1 ?a ?b) ⇒ (R2 (R3 (R4 (R1 ?a ?b) ?b)) ?a)` | 5 |
| premises 0, conclusions 0 | 9 | 6 | 6 | 0 | `[1] (eq c1 ?P1) ⇒ (false)` | 3 |
| premises 1, conclusions 3 | 8 | 8 | 4 | 6 | `[0] (R1 ?a ?b) ⇒ (R2 (R3 (R1 ?a ?b) ?b) ?a)` | 4 |
| premises 2, conclusions 4 or more | 8 | 8 | 8 | 7 | `[0] (R1 ?a ?b (R2 ?c ?d) (R3 ?e ?f) ?g) ⇒ (R4 (R5 ?a ?b ?d ?g (R3 (R3 ?e ?f) ?c)) (R3 (R3 ?e ?f) ?c) ?b c1 c2)` | 1 |
| premises 4 or more, conclusions 4 or more | 8 | 8 | 8 | 8 | `[0] (R1 (R2 (R2 (R2 (R2 (R2 (R2 (R2 (R2 (R2 (R2 ?a ?b) ?c) ?d) ?e) ?f) ?g) ?h) ?i) ?j) ?k)) ⇒ (R3 (R2 (R2 (…` | 1 |

The 5 smaller groups are in `groups.tsv`.

### By builds or takes apart

| builds or takes apart | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| flat: nothing nested | 751 | 444 | 291 | 72 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| concludes no fact | 264 | 144 | 101 | 23 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| level: nested alike | 201 | 196 | 153 | 96 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 (R4 ?a ?b ?c) ?b)` | 10 |
| takes apart: the premises nested deeper | 178 | 170 | 105 | 81 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| builds: a conclusion nested deeper than any premise | 116 | 113 | 90 | 64 | `[0] (R1 ?a ?b) ⇒ (R2 (R3 ?a ?b) ?a)` | 10 |
| no premise | 43 | 37 | 25 | 5 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | 10 |

### By join structure

| join structure | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| one premise | 947 | 696 | 429 | 234 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| a pair | 241 | 140 | 112 | 48 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| a product: premises that share no variable | 179 | 111 | 88 | 25 | `[0] (and (R1 c1) (R1 c2)) ⇒ (false)` | 20 |
| cyclic | 55 | 53 | 53 | 19 | `[0] (and (R1 ?a ?b ?c ?d ?e) (R2 ?b ?f ?g ?h) (R3 ?i ?d ?g) (R3 ?i ?c ?f)) ⇒ (R3 ?i ?e ?h)` | 2 |
| no premise | 52 | 43 | 31 | 5 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | 10 |
| a chain | 34 | 24 | 23 | 3 | `[0] (and (R1 ?a ?b ?c) (R2 ?c) (R3 ?a)) ⇒ (false)` | 3 |
| a star: one variable in every premise | 33 | 27 | 19 | 2 | `[0] (and (R1 ?a) (R2 ?a) (R3 ?a)) ⇒ (R4 ?a)` | 5 |
| a tree | 12 | 10 | 10 | 5 | `[2] (and (?P1 ?a ?b) (?P1 ?c ?d) (R1 ?a ?P2) (R2 ?c ?a) (absent (and (?P1 ?a ?e) (neq ?b ?e)))) ⇒ (R2 ?d ?b)` | 3 |

### By join components

| join components | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| 0 | 52 | 43 | 31 | 5 | `[0] (unknown (R1 c1 c2)) ⇒ (R2 c1 c2)` | 10 |
| 1 | 1322 | 950 | 646 | 311 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| 2 | 153 | 91 | 68 | 19 | `[0] (and (R1 c1) (R1 c2)) ⇒ (false)` | 20 |
| 3 | 20 | 14 | 14 | 5 | `[0] (and (R1 ?a) (R2 ?b) (R3 ?c)) ⇒ (false)` | 3 |
| 4 or more | 6 | 6 | 6 | 1 | `[0] (and (R1 ?a) (R1 ?b) (R1 ?c) (R1 ?d) (R1 ?e) (R1 ?f) (R1 ?g) (R1 ?h) (R1 ?i) (R1 ?j) (R1 ?k) (R1 ?l) (R…` | 1 |

### By feeds itself

| feeds itself | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| no conclusion's relation is read | 1110 | 838 | 564 | 276 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| concludes no fact | 264 | 144 | 101 | 23 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| every conclusion's relation is read | 164 | 111 | 90 | 42 | `[1] (?P1 ?a ?b) ⇒ (?P1 ?b ?a)` | 16 |
| some conclusions' relations are read | 15 | 11 | 10 | 0 | `[0] (R1 ?a ?b ?c ?d) ⇒ (and (R1 ?c ?d ?a ?b) (R2 ?a ?b ?c ?d))` | 5 |

### By conclusion variables

| conclusion variables | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| one premise, one conclusion: nested | 354 | 343 | 229 | 184 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| concludes no fact | 264 | 144 | 101 | 23 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| from several premises (a join's output) | 239 | 163 | 144 | 57 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?a) (neq ?b ?c)) ⇒ (?P1 ?c ?b)` | 16 |
| all from one premise | 226 | 148 | 126 | 36 | `[0] (and (R1 ?a) (R2 ?a)) ⇒ (R3 ?a)` | 15 |
| one premise, one conclusion: a copy | 201 | 113 | 43 | 7 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| ground: no variable in a conclusion | 84 | 67 | 42 | 6 | `[0] (R1 ?a) ⇒ (R2 c1)` | 11 |
| one premise, one conclusion: a projection | 70 | 61 | 33 | 18 | `[0] (R1 ?a ?b) ⇒ (R2 ?a)` | 8 |
| one premise, one conclusion: a constant put in | 36 | 32 | 20 | 3 | `[0] (and (R1 ?a) (forall ?b (R2 ?a ?b) (R3 ?b))) ⇒ (R4 ?a c1)` | 5 |
| one premise, one conclusion: a permutation | 25 | 7 | 6 | 4 | `[1] (?P1 ?a ?b) ⇒ (?P1 ?b ?a)` | 16 |
| one premise, one conclusion: with a parameter | 21 | 5 | 5 | 0 | `[4] (?P1 ?P2 ?a) ⇒ (?P3 ?P4 ?a)` | 7 |
| a variable no positive premise binds | 18 | 12 | 11 | 3 | `[4] (not (?P1 ?P2 ?a)) ⇒ (not (?P3 ?P4 ?a))` | 6 |
| parameters alone | 15 | 9 | 5 | 0 | `[1] (R1 ?P1) ⇒ (R2 ?P1)` | 10 |

### By schema

| schema | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| none of the schemas | 1141 | 923 | 696 | 319 | `[0] (R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` | 24 |
| an hrule | 124 | 56 | 32 | 0 | `hrule [0] (R1 ?a) ⇒ (R2 ?a)` | 52 |
| unary inclusion: R(a) ⇒ S(a) | 81 | 48 | 7 | 2 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| empty: R(a,b) ⇒ false | 28 | 6 | 3 | 1 | `[0] (R1 ?a c1) ⇒ (false)` | 17 |
| unary empty: R(a) ⇒ false | 28 | 13 | 3 | 1 | `[0] (R1 ?a) ⇒ (false)` | 16 |
| functional: R(a,b), R(a,c), b≠c ⇒ false | 25 | 3 | 2 | 1 | `[1] (and (?P1 ?a ?b) (?P1 ?a ?c) (neq ?b ?c)) ⇒ (false)` | 23 |
| default: R(a), absent S(a) ⇒ T(a) | 20 | 9 | 3 | 0 | `[0] (and (R1 ?a) (absent (R2 ?a))) ⇒ (R3 ?a)` | 18 |
| symmetry: R(a,b) ⇒ R(b,a) | 19 | 3 | 2 | 2 | `[1] (?P1 ?a ?b) ⇒ (?P1 ?b ?a)` | 16 |
| transitivity with a guard: R(a,b), R(b,c), a≠c ⇒ R(a,c) | 16 | 1 | 1 | 1 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?a) (neq ?b ?c)) ⇒ (?P1 ?c ?b)` | 16 |
| inclusion: R(a,b) ⇒ S(a,b) | 15 | 9 | 4 | 6 | `[2] (?P1 ?a ?b) ⇒ (?P2 ?a ?b)` | 6 |
| unary intersection: R(a), S(a) ⇒ T(a) | 15 | 9 | 1 | 0 | `[0] (and (R1 ?a) (R2 ?a)) ⇒ (R3 ?a)` | 15 |
| domain: R(a,b) ⇒ S(a) | 12 | 10 | 2 | 1 | `[0] (R1 ?a ?b) ⇒ (R2 ?a)` | 8 |
| unary disjoint: R(a), S(a) ⇒ false | 11 | 5 | 1 | 0 | `[0] (and (R1 ?a) (R2 ?a)) ⇒ (false)` | 11 |
| injective: R(a,c), R(b,c), a≠b ⇒ false | 9 | 1 | 1 | 1 | `[1] (and (?P1 ?a ?b) (?P1 ?c ?b) (neq ?a ?c)) ⇒ (false)` | 9 |

The 6 smaller groups are in `groups.tsv`.

### By rule or hrule

| rule or hrule | rules | distinct | shapes (ii) | in `stdlib/` | its most frequent shape | rules of it |
|---|---:|---:|---:|---:|---|---:|
| rule | 1429 | 1048 | 733 | 341 | `[0] (R1 ?a) ⇒ (R2 ?a)` | 60 |
| hrule | 124 | 56 | 32 | 0 | `hrule [0] (R1 ?a) ⇒ (R2 ?a)` | 52 |

### Two keys at once

**premises (rows) by conclusions (columns)**, in rules:

|  | 0 | 1 | 2 | 3 | 4 or more | all |
|---|---:|---:|---:|---:|---:|---:|
| **0** | · | 39 | · | · | 13 | 52 |
| **1** | 1 | 898 | 24 | 12 | 12 | 947 |
| **2** | · | 348 | 7 | · | 2 | 357 |
| **3** | · | 104 | 3 | · | · | 107 |
| **4** | · | 29 | 4 | · | · | 33 |
| **5** | · | 24 | · | · | · | 24 |
| **6 or more** | · | 33 | · | · | · | 33 |

**premises (rows) by relations (columns)**, in rules:

|  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 or more | all |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **0** | 6 | 13 | 29 | 3 | 1 | · | · | · | · | 52 |
| **1** | · | 111 | 381 | 236 | 135 | 71 | 11 | · | 2 | 947 |
| **2** | · | 77 | 101 | 104 | 48 | 26 | · | · | 1 | 357 |
| **3** | · | 2 | 9 | 49 | 22 | 23 | 1 | 1 | · | 107 |
| **4** | · | 1 | 4 | 14 | 3 | 4 | 5 | 2 | · | 33 |
| **5** | · | · | · | 4 | 2 | 11 | 7 | · | · | 24 |
| **6 or more** | · | · | 3 | 5 | 7 | · | 12 | 2 | 4 | 33 |

**premises (rows) by variables (columns)**, in rules:

|  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 or more | all |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **0** | 33 | 10 | 2 | 1 | · | 6 | · | · | · | · | · | 52 |
| **1** | 32 | 334 | 177 | 131 | 130 | 81 | 24 | 11 | 19 | 3 | 5 | 947 |
| **2** | 31 | 47 | 40 | 26 | 73 | 51 | 16 | 18 | 27 | 18 | 10 | 357 |
| **3** | 1 | 11 | 7 | 16 | 11 | 12 | 20 | 10 | 1 | 17 | 1 | 107 |
| **4** | · | · | · | · | 1 | 3 | 9 | 9 | 7 | 4 | · | 33 |
| **5** | · | 3 | · | · | · | 3 | 7 | 5 | 2 | 4 | · | 24 |
| **6 or more** | · | 5 | · | · | · | · | 2 | 3 | · | · | 23 | 33 |

**join structure (rows) by feeds itself (columns)**, in rules:

|  | concludes no fact | every conclusion's relation is read | no conclusion's relation is read | some conclusions' relations are read | all |
|---|---:|---:|---:|---:|---:|
| **a chain** | 14 | 9 | 11 | · | 34 |
| **a pair** | 67 | 47 | 123 | 4 | 241 |
| **a product: premises that share no variable** | 44 | 23 | 112 | · | 179 |
| **a star: one variable in every premise** | 3 | 4 | 26 | · | 33 |
| **a tree** | 1 | 8 | 3 | · | 12 |
| **cyclic** | · | 29 | 25 | 1 | 55 |
| **no premise** | 9 | · | 43 | · | 52 |
| **one premise** | 126 | 44 | 767 | 10 | 947 |

**builds or takes apart (rows) by join structure (columns)**, in rules:

|  | a chain | a pair | a product: premises that share no variable | a star: one variable in every premise | a tree | cyclic | no premise | one premise | all |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **builds: a conclusion nested deeper than any premise** | · | 21 | 11 | · | · | 4 | · | 80 | 116 |
| **concludes no fact** | 14 | 67 | 44 | 3 | 1 | · | 9 | 126 | 264 |
| **flat: nothing nested** | 17 | 114 | 116 | 25 | 10 | 21 | · | 448 | 751 |
| **level: nested alike** | 1 | 31 | 2 | · | · | 26 | · | 141 | 201 |
| **no premise** | · | · | · | · | · | · | 43 | · | 43 |
| **takes apart: the premises nested deeper** | 2 | 8 | 6 | 5 | 1 | 4 | · | 152 | 178 |

**schema (rows) by parameters (columns)**, in rules:

|  | 0 | 1 | 2 | 3 | 4 | 5 | all |
|---|---:|---:|---:|---:|---:|---:|---:|
| **an hrule** | 100 | 1 | 10 | 13 | · | · | 124 |
| **asymmetric: R(a,b), R(b,a) ⇒ false** | · | 1 | · | · | · | · | 1 |
| **composition: R(a,b), S(b,c) ⇒ T(a,c)** | · | · | · | 1 | · | · | 1 |
| **converse: R(a,b) ⇒ S(b,a)** | · | · | 2 | · | · | · | 2 |
| **default: R(a), absent S(a) ⇒ T(a)** | 18 | 2 | · | · | · | · | 20 |
| **domain: R(a,b) ⇒ S(a)** | 12 | · | · | · | · | · | 12 |
| **empty: R(a,b) ⇒ false** | 27 | 1 | · | · | · | · | 28 |
| **functional: R(a,b), R(a,c), b≠c ⇒ false** | 2 | 23 | · | · | · | · | 25 |
| **inclusion: R(a,b) ⇒ S(a,b)** | 7 | · | 6 | 2 | · | · | 15 |
| **injective: R(a,c), R(b,c), a≠b ⇒ false** | · | 9 | · | · | · | · | 9 |
| **intersection: R(a,b), S(a,b) ⇒ T(a,b)** | 2 | · | · | 1 | · | · | 3 |
| **irreflexive: R(a,a) ⇒ false** | · | 1 | · | · | · | · | 1 |
| **none of the schemas** | 939 | 59 | 42 | 15 | 26 | 60 | 1141 |
| **symmetry: R(a,b) ⇒ R(b,a)** | 3 | 16 | · | · | · | · | 19 |
| **transitivity with a guard: R(a,b), R(b,c), a≠c ⇒ R(a,c)** | · | 16 | · | · | · | · | 16 |
| **transitivity: R(a,b), R(b,c) ⇒ R(a,c)** | · | 1 | · | · | · | · | 1 |
| **unary disjoint: R(a), S(a) ⇒ false** | 11 | · | · | · | · | · | 11 |
| **unary empty: R(a) ⇒ false** | 17 | 11 | · | · | · | · | 28 |
| **unary inclusion: R(a) ⇒ S(a)** | 61 | 16 | 4 | · | · | · | 81 |
| **unary intersection: R(a), S(a) ⇒ T(a)** | 15 | · | · | · | · | · | 15 |

### Which grouping refines which

Grouping A *refines* B when two rules A puts together B puts together too, which is checked here on the census's rules and is no proof. Every key is refined by level (ii) by construction. A `✓` says the row's level refines the column's key:

| level | relations | arity profile | arguments in all | widest atom | premises × conclusions | variables | variables a premise | constants | parameters | negation profile | guard profile | nesting | builds or takes apart | join structure | join components | feeds itself | conclusion variables | schema | rule or hrule |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| (ii) | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| (iii) | ✓ | ✓ | ✓ | ✓ | ✓ | · | · | · | · | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | · | ✓ | ✓ |
| (iii′) | ✓ | ✓ | ✓ | ✓ | ✓ | · | · | · | · | ✓ | ✓ | ✓ | ✓ | · | · | ✓ | · | · | ✓ |
| (iv) | ✓ | · | · | · | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | · | ✓ | ✓ | · | · | ✓ | · | · | ✓ |
| (iv′) | · | · | · | · | · | · | · | · | ✓ | · | · | · | · | · | · | · | · | · | ✓ |

Between the keys: *arity profile* refines *arguments in all*; *nesting* refines *builds or takes apart*; *schema* refines *rule or hrule*.

Between the levels (the row refines the column):

| level | classes | the same text | the same rule | relations renamed | constants renamed | (i) | (ii) | (iii) | (iii′) | (iv) | (iv′) |
|---|---:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| the same text | 1105 | — | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| the same rule | 1104 | · | — | ✓ | ✓ | · | ✓ | ✓ | ✓ | ✓ | ✓ |
| relations renamed | 845 | · | · | — | · | · | ✓ | ✓ | ✓ | ✓ | ✓ |
| constants renamed | 1074 | · | · | · | — | · | ✓ | ✓ | ✓ | ✓ | ✓ |
| (i) | 770 | · | · | · | · | — | ✓ | ✓ | ✓ | ✓ | ✓ |
| (ii) | 765 | · | · | · | · | · | — | ✓ | ✓ | ✓ | ✓ |
| (iii) | 708 | · | · | · | · | · | · | — | ✓ | · | · |
| (iii′) | 572 | · | · | · | · | · | · | · | — | · | · |
| (iv) | 627 | · | · | · | · | · | · | · | · | — | ✓ |
| (iv′) | 123 | · | · | · | · | · | · | · | · | · | — |

## B. Connections

Scopes. *Global*: any two of the 1 553 rules, 1 205 128 unordered pairs; a name is equal to a name by spelling alone, across programs that have nothing to do with each other. *File*: two rules of one file, 17 238 unordered pairs over 402 files. *Program*: a rule a file writes and a rule its imports bring, 36 533 pairs over the 216 files whose imports bring a rule. A pair of imported rules is counted once, in its module's file scope, and not again for every program that imports it.

### The catalogue

Every kind is a relation on rules. `—` is symmetric (unordered pairs), `→` directed (ordered pairs, A to B; a rule connected to itself is counted in *self* and in no other column). *Density* is the pairs over the pairs the scope has. *Program* is pairs of a file's rule and an imported one; for a directed kind, *own → imported / imported → own*. *None* is the number of rules the kind connects to no other rule, globally; *degree* is a rule's number of partners globally (out-degree for `→`): least / median / mean / 90th percentile / greatest.

| kind | definition |  | global pairs | density | file pairs | density | program pairs | self | none | degree |
|---|---|:-:|---:|---:|---:|---:|---:|---:|---:|---|
| 1a | share a relation name (a name either writes at the head of a form, at any depth) | — | 46 682 | 3.87 % | 6 040 | 35.04 % | 4 783 |  | 198 | 0 / 41 / 60.1 / 145 / 262 |
| 1b | share a constant (a name either writes as an argument) | — | 45 640 | 3.79 % | 4 192 | 24.32 % | 7 822 |  | 932 | 0 / 0 / 58.8 / 249 / 345 |
| 1c | one's relation is the other's constant (a relation named as a value) | — | 5 156 | 0.43 % | 486 | 2.82 % | 286 |  | 513 | 0 / 1 / 6.6 / 14 / 407 |
| 1d | share any name | — | 87 252 | 7.24 % | 8 334 | 48.35 % | 11 086 |  | 178 | 0 / 62 / 112.4 / 306 / 421 |
| 1e | A writes B's rule name, as a relation or as a constant | → | 6 519 | 0.27 % | 656 | 1.90 % | 21 / 0 | 24 | 855 | 0 / 0 / 4.2 / 11 / 99 |
| 1f | A writes B's rule name as a constant (names the rule as a value) | → | 1 838 | 0.08 % | 433 | 1.26 % | 9 / 0 | 19 | 1216 | 0 / 0 / 1.2 / 3 / 99 |
| 2a | the same relation in the same clause role (any of the five below) | — | 27 977 | 2.32 % | 3 429 | 19.89 % | 2 472 |  | 210 | 0 / 20 / 36.0 / 109 / 176 |
| 2b | … both read it as a positive premise (= 7a) | — | 23 253 | 1.93 % | 2 850 | 16.53 % | 1 839 |  | 325 | 0 / 13 / 29.9 / 85 / 148 |
| 2c | … both read it under `absent` | — | 678 | 0.06 % | 57 | 0.33 % | 0 |  | 1440 | 0 / 0 / 0.9 / 0 / 27 |
| 2d | … both read its stored negation, `(not (R …))` | — | 11 | 0.00 % | 1 | 0.01 % | 0 |  | 1546 | 0 / 0 / 0.0 / 0 / 4 |
| 2e | … both conclude it (= 6a) | — | 6 352 | 0.53 % | 1 167 | 6.77 % | 681 |  | 629 | 0 / 2 / 8.2 / 36 / 65 |
| 2f | … both conclude its negation | — | 3 | 0.00 % | 0 | 0.00 % | 0 |  | 1547 | 0 / 0 / 0.0 / 0 / 1 |
| 2g | the same constant at the same argument place of the same relation, either clause | — | 2 595 | 0.22 % | 785 | 4.55 % | 203 |  | 1061 | 0 / 0 / 3.3 / 11 / 49 |
| 2h | … both in the `:match` | — | 1 222 | 0.10 % | 420 | 2.44 % | 11 |  | 1217 | 0 / 0 / 1.6 / 7 / 24 |
| 2i | … both in the `:assert` | — | 1 333 | 0.11 % | 429 | 2.49 % | 136 |  | 1310 | 0 / 0 / 1.7 / 3 / 39 |
| 2j | … A's `:assert`, B's `:match` | → | 242 | 0.01 % | 103 | 0.30 % | 43 / 14 | 18 | 1426 | 0 / 0 / 0.2 / 0 / 11 |
| 3a | A concludes a relation B reads as a positive premise, by name | → | 11 094 | 0.46 % | 1 905 | 5.53 % | 458 / 426 | 118 | 713 | 0 / 1 / 7.1 / 19 / 120 |
| 3b | … by name and number of arguments | → | 9 560 | 0.40 % | 1 903 | 5.52 % | 458 / 426 | 118 | 731 | 0 / 1 / 6.2 / 19 / 120 |
| 3c | … a conclusion of A unifies with a premise of B (both headed by a name) | → | 8 540 | 0.35 % | 1 480 | 4.29 % | 369 / 387 | 102 | 739 | 0 / 1 / 5.5 / 10 / 84 |
| 3d | … unifies, one of the two headed by a variable (a generic rule) | → | 153 764 | 6.38 % | 1 214 | 3.52 % | 502 / 390 | 137 | 755 | 0 / 16 / 99.0 / 223 / 747 |
| 4a | A concludes a relation B tests with `absent`, by name | → | 1 283 | 0.05 % | 46 | 0.13 % | 0 / 0 | 6 | 1367 | 0 / 0 / 0.8 / 1 / 28 |
| 4b | … a conclusion of A unifies with an atom B tests with `absent` | → | 1 218 | 0.05 % | 46 | 0.13 % | 0 / 0 | 6 | 1377 | 0 / 0 / 0.8 / 1 / 27 |
| 4c | A concludes `(not (R …))` and B reads `(not (R …))`, by name | → | 17 | 0.00 % | 3 | 0.01 % | 0 / 0 | 1 | 1546 | 0 / 0 / 0.0 / 0 / 5 |
| 4d | … the two unify | → | 17 | 0.00 % | 3 | 0.01 % | 0 / 0 | 1 | 1546 | 0 / 0 / 0.0 / 0 / 5 |
| 4e | A concludes `(not (R …))` and B tests `(absent (not (R …)))`, unifying | → | 2 | 0.00 % | 1 | 0.00 % | 0 / 0 | 0 | 1551 | 0 / 0 / 0.0 / 0 / 1 |
| 4f | A concludes `(R …)` and B reads `(not (R …))`, unifying: opposite signs | → | 200 | 0.01 % | 4 | 0.01 % | 0 / 0 | 3 | 1505 | 0 / 0 / 0.1 / 0 / 5 |
| 4g | A concludes `(not (R …))` and B reads `(R …)`, unifying: opposite signs | → | 207 | 0.01 % | 2 | 0.01 % | 0 / 0 | 4 | 1543 | 0 / 0 / 0.1 / 0 / 84 |
| 5a | A concludes a fact that activates B: headed by B's name, as many arguments as B has parameters | → | 1 217 | 0.05 % | 165 | 0.48 % | 8 / 0 | 0 | 1391 | 0 / 0 / 0.8 / 1 / 32 |
| 5b | A reads B's activator as a premise (a rule about a rule's being switched on) | → | 834 | 0.03 % | 63 | 0.18 % | 2 / 0 | 0 | 1418 | 0 / 0 / 0.5 / 0 / 23 |
| 6a | both conclude the same relation, by name (= 2e) | — | 6 352 | 0.53 % | 1 167 | 6.77 % | 681 |  | 629 | 0 / 2 / 8.2 / 36 / 65 |
| 6b | … two of their conclusions unify (two ways to derive one fact) | — | 3 321 | 0.28 % | 261 | 1.51 % | 67 |  | 863 | 0 / 0 / 4.3 / 11 / 42 |
| 6c | one concludes `(R …)`, the other `(not (R …))`, unifying (a possible conflict) | — | 133 | 0.01 % | 5 | 0.03 % | 0 |  | 1455 | 0 / 0 / 0.2 / 0 / 43 |
| 6d | A's conclusion unifies with a premise of B, and B concludes `(false)` | → | 1 548 | 0.06 % | 113 | 0.33 % | 7 / 4 | 0 | 1339 | 0 / 0 / 1.0 / 3 / 20 |
| 6e | both conclude `(false)` | — | 21 945 | 1.82 % | 206 | 1.20 % | 39 |  | 1343 | 0 / 0 / 28.3 / 209 / 209 |
| 6f | both conclude `(open …)` | — | 378 | 0.03 % | 5 | 0.03 % | 2 |  | 1525 | 0 / 0 / 0.5 / 0 / 27 |
| 7a | both read the same relation as a positive premise, by name (= 2b) | — | 23 253 | 1.93 % | 2 850 | 16.53 % | 1 839 |  | 325 | 0 / 13 / 29.9 / 85 / 148 |
| 7b | … two of their premises unify (one fact can feed both) | — | 16 064 | 1.33 % | 1 347 | 7.81 % | 221 |  | 603 | 0 / 4 / 20.7 / 83 / 144 |
| 7c | … they have a premise in common, the same up to a renaming of variables | — | 33 934 | 2.82 % | 1 788 | 10.37 % | 758 |  | 506 | 0 / 7 / 43.7 / 216 / 290 |
| 7d | the same join: one variable at argument i of R and argument j of S, in both | — | 3 107 | 0.26 % | 422 | 2.45 % | 24 |  | 1214 | 0 / 0 / 4.0 / 11 / 92 |
| 8a | the same rule (variables renamed, any order) | — | 1 560 | 0.13 % | 3 | 0.02 % | 0 |  | 936 | 0 / 0 / 2.0 / 6 / 22 |
| 8b | one rule under a renaming of relations | — | 5 604 | 0.47 % | 209 | 1.21 % | 120 |  | 654 | 0 / 1 / 7.2 / 22 / 59 |
| 8c | one rule under a renaming of constants | — | 1 778 | 0.15 % | 64 | 0.37 % | 0 |  | 902 | 0 / 0 / 2.3 / 7 / 22 |
| 8d | the same shape at level (ii) | — | 6 146 | 0.51 % | 297 | 1.72 % | 123 |  | 562 | 0 / 2 / 7.9 / 22 / 59 |
| 8e | the same shape at level (iii) | — | 8 813 | 0.73 % | 306 | 1.78 % | 125 |  | 516 | 0 / 3 / 11.3 / 29 / 80 |
| 8f | the same shape at level (iii′) | — | 12 396 | 1.03 % | 547 | 3.17 % | 180 |  | 373 | 0 / 5 / 16.0 / 54 / 101 |
| 8g | the same skeleton, level (iv) | — | 7 094 | 0.59 % | 544 | 3.16 % | 181 |  | 399 | 0 / 3 / 9.1 / 25 / 59 |
| 8h | the same `:match` shape | — | 26 866 | 2.23 % | 764 | 4.43 % | 802 |  | 319 | 0 / 7 / 34.6 / 188 / 188 |
| 8i | the same `:assert` shape | — | 79 288 | 6.58 % | 1 340 | 7.77 % | 720 |  | 197 | 0 / 46 / 102.1 / 284 / 284 |
| 8j | one rule under a renaming of relations, and not the same rule (8b less 8a) | — | 4 044 | 0.34 % | 206 | 1.20 % | 120 |  | 1061 | 0 / 0 / 5.2 / 14 / 59 |
| 8k | A's `:match` is a sub-pattern of B's: a substitution of A's variables puts every literal of A among B's (A's heads all names) | → | 4 298 | 0.18 % | 151 | 0.44 % | 0 / 3 | 0 | 1264 | 0 / 0 / 2.8 / 4 / 81 |
| 8l | … where A has a variable at a head (a generic match) | → | 31 331 | 1.30 % | 375 | 1.09 % | 37 / 75 | 0 | 1413 | 0 / 0 / 20.2 / 0 / 655 |
| 8m | each `:match` a sub-pattern of the other, the two not the same up to renaming | — | 162 | 0.01 % | 6 | 0.03 % | 0 |  | 1478 | 0 / 0 / 0.2 / 0 / 38 |
| 9a | in the same file | — | 17 238 | 1.43 % | 17 238 | 100.00 % | 0 |  | 209 | 0 / 7 / 22.2 / 70 / 88 |
| 9b | in the same directory | — | 159 476 | 13.23 % | 17 238 | 100.00 % | 16 379 |  | 3 | 0 / 163 / 205.4 / 367 / 367 |
| 9c | in the scope of one program that imports (a file's rules and what its imports bring) | — | 63 122 | 5.24 % | 15 582 | 90.39 % | 36 533 |  | 686 | 0 / 6 / 81.3 / 224 / 528 |
| 9d | named together by one rule (a closing rule's list): both names in one rule's text, all three in one program's scope | — | 12 725 | 1.06 % | 12 419 | 72.04 % | 305 |  | 994 | 0 / 0 / 16.4 / 70 / 175 |
| 9e | named together in one `(solve :hrules …)` list | — | 1 | 0.00 % | 1 | 0.01 % | 0 |  | 1551 | 0 / 0 / 0.0 / 0 / 1 |
| 9f | activated by written facts of one file (both rules' activators written in it) | — | 204 | 0.02 % | 169 | 0.98 % | 24 |  | 1439 | 0 / 0 / 0.3 / 0 / 17 |
| 11a | the same rule name, in two files | — | 4 795 | 0.40 % | 0 | 0.00 % | 0 |  | 744 | 0 / 1 / 6.2 / 16 / 60 |
| 11b | … and not the same rule (the name reused for another text) | — | 3 337 | 0.28 % | 0 | 0.00 % | 0 |  | 1126 | 0 / 0 / 4.3 / 9 / 60 |
| 11c | the same rule name twice in one file | — | 0 | 0.00 % | 0 | 0.00 % | 0 |  | 1553 | 0 / 0 / 0.0 / 0 / 0 |

8k to 8m compare the 863 distinct `:match`es that are a flat conjunction of literals (143 rules have an `or`, a nested connective or a macro call that is left out, 21 an empty match, which is a sub-pattern of everything and is left out); 2628 ordered pairs of them subsume, and 0 comparisons ran out of their budget and are counted as no.

### Examples

Three pairs a kind, a pair inside one file where there is one.

| kind | pairs |
|---|---|
| 1a | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:self-converse-is-symmetric`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 1b | `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:add-column-first`; `stdlib/elim.ein:domain-elimination` — `stdlib/elim.ein:no-room-left`; `stdlib/int.ein:int-norm-walks` — `stdlib/int.ein:int-norm-plus` |
| 1c | `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:arith`; `stdlib/int.ein:int-operand` — `stdlib/int.ein:int-arith`; `stdlib/num9.ein:add9-fits` — `stdlib/num9.ein:arith9` |
| 1d | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:self-converse-is-symmetric`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 1e | `stdlib/algebra.ein:symmetric-is-self-converse` → `stdlib/algebra.ein:converse`; `stdlib/arith.ein:arith` → `stdlib/arith.ein:operand-zero`; `stdlib/bijection.ein:bijective-setup` → `stdlib/bijection.ein:functional-negative` |
| 1f | `stdlib/arith.ein:arith` → `stdlib/arith.ein:operand-zero`; `stdlib/int.ein:int-arith` → `stdlib/int.ein:int-operand`; `stdlib/num9.ein:arith9` → `stdlib/num9.ein:add9-fits` |
| 2a | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:converse-pair-symmetric`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 2b | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:symmetric-negative-setup`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 2c | `examples/binary_logic/post.ein:inside-zero` — `examples/binary_logic/post.ein:inside-one`; `examples/branching/12_typed_blind_solve.ein:domain-elimination` — `examples/branching/12_typed_blind_solve.ein:no-room-left`; `examples/domain_elim/ab.ein:domain-elimination` — `examples/domain_elim/ab.ein:range-elimination` |
| 2d | `examples/ein-bugs/alive-empty-interlayer.ein:mutex` — `examples/ein-bugs/alive-empty-interlayer.ein:totality`; `examples/ein-bugs/wrong-answers/forall-core-elim.ein:elim` — `examples/ein-bugs/wrong-answers/forall-core-false.ein:full` |
| 2e | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:converse-pair-symmetric`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/int.ein:int-operand-second` — `stdlib/int.ein:int-simplify-ask` |
| 2f | `examples/ein-bugs/d3-unsat-core.ein:fire-0` — `examples/ein-bugs/unbound-assert-var.ein:fire-3`; `examples/ein-bugs/wrong-answers/forall-core-elim.ein:ex` — `examples/ein-bugs/wrong-answers/forall-core-false.ein:ex`; `examples/ein-bugs/wrong-answers/pair-core-root-derived.ein:neg` — `examples/ein-bugs/wrong-answers/pair-core-root-given.ein:neg` |
| 2g | `stdlib/arith.ein:add-column-first` — `stdlib/arith.ein:add-end`; `stdlib/elim.ein:domain-elimination` — `stdlib/elim.ein:no-room-left`; `stdlib/int.ein:int-norm-walks` — `stdlib/int.ein:int-div-rest` |
| 2h | `stdlib/arith.ein:add-column-first` — `stdlib/arith.ein:add-end`; `stdlib/elim.ein:domain-elimination` — `stdlib/elim.ein:no-room-left`; `stdlib/int.ein:int-div-by-zero` — `stdlib/int.ein:int-div-pp` |
| 2i | `stdlib/arith.ein:add-carry-out` — `stdlib/arith.ein:strip-top`; `stdlib/int.ein:int-norm-walks` — `stdlib/int.ein:int-div-rest`; `stdlib/num9.ein:mul9-start` — `stdlib/num9.ein:mul9-step-zero` |
| 2j | `stdlib/arith.ein:add-column-first` → `stdlib/arith.ein:add-end`; `stdlib/int.ein:int-norm-plus` → `stdlib/int.ein:int-div-by-zero`; `stdlib/num9.ein:mul9-start` → `stdlib/num9.ein:mul9-step` |
| 3a | `stdlib/algebra.ein:symmetric-is-self-converse` → `stdlib/algebra.ein:self-converse-is-symmetric`; `stdlib/arith.ein:operand-zero` → `stdlib/arith.ein:add-ask-second`; `stdlib/bijection.ein:bijective-setup` → `stdlib/bijection.ein:domain-elimination` |
| 3b | `stdlib/algebra.ein:symmetric-is-self-converse` → `stdlib/algebra.ein:self-converse-is-symmetric`; `stdlib/arith.ein:operand-zero` → `stdlib/arith.ein:add-ask-second`; `stdlib/int.ein:int-operand` → `stdlib/int.ein:int-operand-second` |
| 3c | `stdlib/algebra.ein:symmetric-is-self-converse` → `stdlib/algebra.ein:self-converse-is-symmetric`; `stdlib/arith.ein:operand-zero` → `stdlib/arith.ein:add-ask-second`; `stdlib/int.ein:int-operand` → `stdlib/int.ein:int-operand-second` |
| 3d | `stdlib/algebra.ein:converse` → `stdlib/algebra.ein:imply2-fwd`; `stdlib/bijection.ein:bijective-setup` → `stdlib/bijection.ein:functional-negative`; `stdlib/elim.ein:domain-elimination` → `stdlib/elim.ein:typecheck-arg-0` |
| 4a | `examples/branching/13_lookahead_naf_world.ein:promote` → `examples/branching/13_lookahead_naf_world.ein:self-block`; `examples/branching/14_lookahead_unjudgeable.ein:auto-sign` → `examples/branching/14_lookahead_unjudgeable.ein:premature-signoff`; `examples/ein-bugs/activator-all-int.ein:check` → `examples/ein-bugs/activator-all-int.ein:flag` |
| 4b | `examples/branching/13_lookahead_naf_world.ein:promote` → `examples/branching/13_lookahead_naf_world.ein:self-block`; `examples/branching/14_lookahead_unjudgeable.ein:auto-sign` → `examples/branching/14_lookahead_unjudgeable.ein:premature-signoff`; `examples/ein-bugs/activator-all-int.ein:check` → `examples/ein-bugs/activator-all-int.ein:flag` |
| 4c | `examples/ein-bugs/wrong-answers/forall-core-elim.ein:ex` → `examples/ein-bugs/wrong-answers/forall-core-elim.ein:elim`; `examples/ein-bugs/wrong-answers/forall-core-false.ein:ex` → `examples/ein-bugs/wrong-answers/forall-core-false.ein:full`; `examples/features/14_unicode_text.ein:taken` → `examples/features/14_unicode_text.ein:last-house` |
| 4d | `examples/ein-bugs/wrong-answers/forall-core-elim.ein:ex` → `examples/ein-bugs/wrong-answers/forall-core-elim.ein:elim`; `examples/ein-bugs/wrong-answers/forall-core-false.ein:ex` → `examples/ein-bugs/wrong-answers/forall-core-false.ein:full`; `examples/features/14_unicode_text.ein:taken` → `examples/features/14_unicode_text.ein:last-house` |
| 4e | `examples/ein-bugs/d3-unsat-core.ein:fire-0` → `examples/ein-bugs/d3-unsat-core.ein:fire-1` |
| 4f | `examples/ein-bugs/alive-empty-interlayer.ein:guess` → `examples/ein-bugs/alive-empty-interlayer.ein:mutex`; `examples/ein-bugs/alive-empty-phase1.ein:guess` → `examples/ein-bugs/alive-empty-phase1.ein:totality`; `examples/ein-bugs/complete-records-stale.ein:guess-p` → `examples/ein-bugs/complete-records-stale.ein:totality` |
| 4g | `examples/ein-bugs/d3-unsat-core.ein:fire-0` → `examples/ein-bugs/d3-unsat-core.ein:fire-1`; `examples/ein-bugs/wrong-answers/pair-core-root-given.ein:neg` → `examples/ein-bugs/wrong-answers/pair-core-root-given.ein:g`; `examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein:w` → `examples/ein-bugs/alt-cap-core-reordered.ein:narrow` |
| 5a | `stdlib/algebra.ein:symmetric-is-self-converse` → `stdlib/algebra.ein:converse`; `stdlib/bijection.ein:bijective-setup` → `stdlib/bijection.ein:functional-negative`; `stdlib/slots.ein:slot-partition-setup` → `stdlib/slots.ein:slot-locate` |
| 5b | `stdlib/algebra.ein:symmetric-is-self-converse` → `stdlib/algebra.ein:symmetric`; `examples/branching/12_typed_blind_solve.ein:derive-functional-negative` → `examples/branching/12_typed_blind_solve.ein:functional`; `examples/domain_elim/ab.ein:derive-functional-negative` → `examples/domain_elim/ab.ein:functional` |
| 6a | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:converse-pair-symmetric`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/int.ein:int-operand-second` — `stdlib/int.ein:int-simplify-ask` |
| 6b | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:converse-pair-symmetric`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-name`; `stdlib/int.ein:int-norm-plus` — `stdlib/int.ein:int-norm-minus` |
| 6c | `examples/ein-bugs/d3-unsat-core.ein:fire-0` — `examples/ein-bugs/d3-unsat-core.ein:fire-1`; `examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein:v` — `examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein:w`; `examples/ein-bugs/wrong-answers/forall-core-elim.ein:ex` — `examples/ein-bugs/wrong-answers/forall-core-elim.ein:elim` |
| 6d | `stdlib/algebra.ein:symmetric-is-self-converse` → `stdlib/algebra.ein:converse-illtyped-dom`; `stdlib/arith.ein:div-by-zero` → `stdlib/arith.ein:division-by-zero-is-false`; `examples/beyond_arith/cryptarithm.ein:guess` → `examples/beyond_arith/cryptarithm.ein:column-wrong` |
| 6e | `stdlib/algebra.ein:converse-illtyped-dom` — `stdlib/algebra.ein:converse-illtyped-ran`; `stdlib/bijection.ein:typecheck-arg-0` — `stdlib/bijection.ein:typecheck-arg-1`; `stdlib/elim.ein:typecheck-arg-0` — `stdlib/elim.ein:typecheck-arg-1` |
| 6f | `stdlib/algebra.ein:total-owed` — `stdlib/algebra.ein:surjective-owed`; `stdlib/slots.ein:slot-owed-room` — `stdlib/slots.ein:slot-owed-fill`; `examples/lattice/synth/local_not_global.ein:owe-1` — `examples/lattice/synth/local_not_global.ein:owe-2` |
| 7a | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:symmetric-negative-setup`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 7b | `stdlib/algebra.ein:symmetric-is-self-converse` — `stdlib/algebra.ein:symmetric-negative-setup`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-name`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 7c | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-fwd`; `stdlib/arith.ein:let-named` — `stdlib/arith.ein:operand-name`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 7d | `stdlib/algebra.ein:converse-illtyped-dom` — `stdlib/algebra.ein:converse-illtyped-ran`; `stdlib/arith.ein:add-column` — `stdlib/arith.ein:add-column-first`; `stdlib/bijection.ein:functional-negative` — `stdlib/bijection.ein:domain-elimination` |
| 8a | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-reverse`; `stdlib/bijection.ein:typecheck-arg-0` — `stdlib/elim.ein:typecheck-arg-0`; `examples/arith/num9.ein:then-divide` — `tests/stdlib/num9/13_chain.ein:then-divide` |
| 8b | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-reverse`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/int.ein:int-simplify-ask` — `stdlib/int.ein:int-neg-ask` |
| 8c | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-reverse`; `stdlib/int.ein:int-div-floor-pn-exact` — `stdlib/int.ein:int-div-floor-np-exact`; `examples/beyond_arith/primes.ein:trial-on-eq` — `examples/beyond_arith/primes.ein:trial-on-gt` |
| 8d | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-reverse`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/int.ein:int-simplify-ask` — `stdlib/int.ein:int-neg-ask` |
| 8e | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-reverse`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/int.ein:int-simplify-ask` — `stdlib/int.ein:int-neg-ask` |
| 8f | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-fwd`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/bijection.ein:functional-negative` — `stdlib/bijection.ein:injective-negative` |
| 8g | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-fwd`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/bijection.ein:functional-negative` — `stdlib/bijection.ein:injective-negative` |
| 8h | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-fwd`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/int.ein:int-norm-plus` — `stdlib/int.ein:int-norm-minus` |
| 8i | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply2-fwd`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/bijection.ein:functional-negative` — `stdlib/bijection.ein:injective-negative` |
| 8j | `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:strip-none`; `stdlib/int.ein:int-simplify-ask` — `stdlib/int.ein:int-neg-ask`; `stdlib/rat.ein:rat-cross-end` — `stdlib/rat.ein:rat-add-answer` |
| 8k | `stdlib/algebra.ein:converse-pair-symmetric` → `stdlib/algebra.ein:self-converse-is-symmetric`; `stdlib/arith.ein:let-named` → `stdlib/arith.ein:operand-name`; `examples/arith/factorial.ein:next-number` → `examples/arith/factorial.ein:next-product` |
| 8l | `stdlib/algebra.ein:converse` → `stdlib/algebra.ein:self-converse-is-symmetric`; `stdlib/slots.ein:slot-exclusive` → `stdlib/slots.ein:slot-occupied`; `stdlib/typing.ein:reflexive-dom` → `stdlib/typing.ein:type-hierarchy-converse` |
| 8m | `examples/beyond_arith/laws.ein:domain` — `examples/beyond_arith/laws.ein:dividends`; `examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein:u-short` — `examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein:w`; `examples/branching/05_mini_zebra.ein:guess` — `examples/lattice/01_subset_pruned.ein:guess` |
| 9a | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply1`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 9b | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply1`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 9c | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply1`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:bijective-setup` — `stdlib/bijection.ein:typecheck-setup` |
| 9d | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:compose`; `stdlib/arith.ein:operand-zero` — `stdlib/arith.ein:operand-number`; `stdlib/bijection.ein:functional-negative` — `stdlib/bijection.ein:injective-negative` |
| 9e | `examples/syntax/query-full.ein:h1` — `examples/syntax/query-full.ein:h2` |
| 9f | `stdlib/algebra.ein:converse` — `stdlib/algebra.ein:imply1`; `stdlib/bijection.ein:functional-negative` — `stdlib/bijection.ein:injective-negative`; `stdlib/elim.ein:typecheck-arg-0` — `stdlib/elim.ein:typecheck-arg-1` |
| 11a | `stdlib/algebra.ein:identity` — `examples/binary_logic/props_rules.ein:identity`; `stdlib/arith.ein:add-column` — `examples/beyond_arith/cryptarithm.ein:add-column`; `stdlib/bijection.ein:functional-negative` — `examples/branching/12_typed_blind_solve.ein:functional-negative` |
| 11b | `stdlib/algebra.ein:identity` — `examples/binary_logic/props_rules.ein:identity`; `stdlib/arith.ein:add-column` — `examples/beyond_arith/cryptarithm.ein:add-column`; `stdlib/bijection.ein:functional-negative` — `examples/branching/12_typed_blind_solve.ein:functional-negative` |
| 11c | none |

### Files alike

Pairs of files by 8j: rules of one that are a rule of the other under a renaming of relations, and not the same rule. The fifteen closest:

| file A | rules | of them with a partner in B | file B | rules | of them with a partner in A |
|---|---:|---:|---|---:|---:|
| `examples/binary_arithmetic/bin.ein` | 68 | 53 | `stdlib/arith.ein` | 89 | 53 |
| `stdlib/int.ein` | 71 | 13 | `stdlib/rat.ein` | 57 | 14 |
| `examples/binary_arithmetic/bin.ein` | 68 | 6 | `stdlib/int.ein` | 71 | 11 |
| `examples/binary_arithmetic/bin.ein` | 68 | 6 | `stdlib/rat.ein` | 57 | 10 |
| `stdlib/arith.ein` | 89 | 6 | `stdlib/int.ein` | 71 | 11 |
| `stdlib/arith.ein` | 89 | 6 | `stdlib/rat.ein` | 57 | 10 |
| `examples/ein-bugs/zebra2-hints.ein` | 23 | 5 | `examples/zebra2-hints.ein` | 38 | 10 |
| `examples/beyond_arith/inverse_addition.ein` | 11 | 4 | `examples/beyond_arith/tables_backwards.ein` | 10 | 4 |
| `examples/beyond_arith/inverse_addition.ein` | 11 | 4 | `stdlib/arith.ein` | 89 | 5 |
| `examples/branching/12_typed_blind_solve.ein` | 10 | 4 | `examples/zebra2-hints.ein` | 38 | 4 |
| `examples/domain_elim/ab.ein` | 12 | 4 | `examples/zebra2-hints.ein` | 38 | 4 |
| `examples/ein-bugs/activator-all-int.ein` | 4 | 4 | `examples/ein-bugs/wrong-answers/forbid-chain-1.ein` | 5 | 5 |
| `examples/ein-bugs/activator-all-int.ein` | 4 | 4 | `examples/ein-bugs/wrong-answers/forbid-chain-2.ein` | 6 | 6 |
| `examples/ein-bugs/activator-all-nested.ein` | 4 | 4 | `examples/ein-bugs/wrong-answers/forbid-chain-1.ein` | 5 | 5 |
| `examples/ein-bugs/activator-all-nested.ein` | 4 | 4 | `examples/ein-bugs/wrong-answers/forbid-chain-2.ein` | 6 | 6 |

Pairs of files by 8a, the same rule in both. The ten closest:

| file A | rules | of them with a partner in B | file B | rules | of them with a partner in A |
|---|---:|---:|---|---:|---:|
| `examples/ein-bugs/zebra2-hints.ein` | 23 | 20 | `examples/zebra2-hints.ein` | 38 | 20 |
| `examples/ein-bugs/zebra2-bad.ein` | 15 | 15 | `examples/zebra2-minus-15.ein` | 15 | 15 |
| `examples/ein-bugs/zebra2-bad.ein` | 15 | 15 | `examples/zebra2.ein` | 15 | 15 |
| `examples/zebra2-minus-15.ein` | 15 | 15 | `examples/zebra2.ein` | 15 | 15 |
| `examples/ein-bugs/zebra2-bad.ein` | 15 | 14 | `examples/zebra2-minus-15-obligations.ein` | 14 | 14 |
| `examples/ein-bugs/zebra2-bad.ein` | 15 | 14 | `examples/zebra2-obligations.ein` | 14 | 14 |
| `examples/zebra2-minus-15-obligations.ein` | 14 | 14 | `examples/zebra2-minus-15.ein` | 15 | 14 |
| `examples/zebra2-minus-15-obligations.ein` | 14 | 14 | `examples/zebra2-obligations.ein` | 14 | 14 |
| `examples/zebra2-minus-15-obligations.ein` | 14 | 14 | `examples/zebra2.ein` | 15 | 14 |
| `examples/zebra2-minus-15.ein` | 15 | 14 | `examples/zebra2-obligations.ein` | 14 | 14 |

743 pairs of files share a rule (8a); 2265 pairs share a rule under a renaming of relations (8j).

### How a rule with parameters is switched on

|  | rules |
|---|---:|
| rules with parameters | 339 |
| … whose activator a file that has the rule in scope writes as a fact | 171 |
| … whose activator a rule in such a scope concludes (5a) | 167 |
| … named in a `(solve :hrules …)` list of such a file | 23 |
| … a written fact and a rule's conclusion both | 28 |
| … none of the three | 6 |

### 10. The assert → match digraph

Nodes are rules; an edge A → B where a conclusion of A unifies with a positive premise of B (3c, both headed by a name), with 3d's edges through a generic rule added in the second column, and by relation name alone (3a) in the third.

**Global** (every rule of the census a node; a name equal by spelling):

|  | 3c | 3c + 3d | 3a |
|---|---:|---:|---:|
| edges | 8 540 | 161 873 | 11 094 |
| rules that feed themselves | 102 | 228 | 118 |
| sources (feed a rule, fed by none) | 211 | 60 | 194 |
| sinks (fed, feed none) | 398 | 404 | 427 |
| isolated | 341 | 19 | 286 |
| strongly connected components of two rules or more | 17 | 3 | 18 |
| the largest of them | 306 | 1 020 | 306 |
| rules on a cycle (a self-loop counted) | 409 | 1 041 | 441 |
| longest path, in components | 9 | 5 | 9 |
| connected components | 363 | 22 | 298 |
| the largest of them | 1 025 | 1 497 | 1 180 |
| out-degree: least / median / mean / 90th / greatest | 0 / 1 / 5.5 / 10 / 84 | 0 / 16 / 104.2 / 230 / 747 | 0 / 1 / 7.1 / 19 / 120 |
| in-degree | 0 / 1 / 5.5 / 12 / 85 | 0 / 53 / 104.2 / 394 / 446 | 0 / 2 / 7.1 / 26 / 88 |

**Per file** (a file's own rules, edges 3c):

- files with a rule: 402; with an edge: 130; with a rule on a cycle: 45; with a component of two rules or more: 9.
- longest path over the files: least / median / mean / 90th / greatest 0 / 0 / 0.4 / 1 / 9.
- largest component over the files: 1 / 1 / 1.3 / 1 / 54.

The twelve files with the longest path:

| file | rules | edges | self | components ≥ 2 | largest | longest path | sources | sinks | isolated |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `examples/word_problems/words.ein` | 37 | 352 | 12 | 4 | 4 | 9 | 1 | 9 | 1 |
| `examples/beyond_arith/inverse_addition.ein` | 11 | 17 | 4 | 0 | 1 | 7 | 1 | 4 | 0 |
| `examples/beyond_arith/column_lemma.ein` | 12 | 31 | 2 | 2 | 2 | 6 | 1 | 3 | 0 |
| `examples/beyond_arith/tables_backwards.ein` | 10 | 12 | 4 | 0 | 1 | 6 | 2 | 2 | 0 |
| `examples/binary_arithmetic/bin.ein` | 68 | 183 | 16 | 2 | 33 | 6 | 6 | 7 | 1 |
| `stdlib/arith.ein` | 89 | 310 | 13 | 2 | 54 | 6 | 6 | 7 | 1 |
| `examples/beyond_arith/equation_by_search.ein` | 5 | 7 | 1 | 0 | 1 | 3 | 1 | 2 | 0 |
| `examples/beyond_arith/equation_no_solution.ein` | 5 | 7 | 1 | 0 | 1 | 3 | 1 | 2 | 0 |
| `examples/binary_logic/adder_against_table.ein` | 6 | 5 | 1 | 0 | 1 | 3 | 1 | 3 | 0 |
| `stdlib/int.ein` | 71 | 62 | 0 | 1 | 2 | 3 | 29 | 17 | 15 |
| `stdlib/rat.ein` | 57 | 54 | 0 | 0 | 1 | 3 | 18 | 18 | 10 |
| `examples/beyond_arith/casting_out_nines.ein` | 10 | 6 | 0 | 0 | 1 | 2 | 2 | 3 | 4 |

The stdlib's modules, each alone:

| module | rules | edges | self | components ≥ 2 | largest | rules on a cycle | longest path | sources | sinks | isolated | connected components |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `stdlib/algebra.ein` | 40 | 14 | 3 | 1 | 3 | 5 | 2 | 0 | 5 | 31 | 32 |
| `stdlib/arith.ein` | 89 | 310 | 13 | 2 | 54 | 65 | 6 | 6 | 7 | 1 | 2 |
| `stdlib/bijection.ein` | 8 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 8 | 8 |
| `stdlib/closure.ein` | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 1 |
| `stdlib/elim.ein` | 4 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 |
| `stdlib/int.ein` | 71 | 62 | 0 | 1 | 2 | 2 | 3 | 29 | 17 | 15 | 19 |
| `stdlib/num9.ein` | 47 | 125 | 12 | 2 | 10 | 12 | 2 | 2 | 5 | 28 | 30 |
| `stdlib/rat.ein` | 57 | 54 | 0 | 0 | 1 | 0 | 3 | 18 | 18 | 10 | 15 |
| `stdlib/slots.ein` | 20 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 20 | 20 |
| `stdlib/typing.ein` | 4 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 |

**Per program** (a file's rules and the rules its imports bring, edges 3c + 3d; 80 files write a rule and import one):

- rules in scope: 2 / 24 / 51.4 / 139 / 219.
- longest path: 0 / 2 / 3.4 / 7 / 11.
- largest component: 1 / 6 / 29.3 / 79 / 152.
- programs with a rule on a cycle: 60.

**Between the stdlib's modules** (3c: a conclusion of a rule of the row's module unifies with a premise of a rule of the column's; the diagonal, in brackets, is the module's own edges with self-loops):

|  | `algebra` | `arith` | `bijection` | `closure` | `elim` | `int` | `num9` | `rat` | `slots` | `typing` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `algebra` → | (17) | · | 2 | 1 | · | · | · | · | · | 2 |
| `arith` → | · | (323) | · | · | · | 86 | · | 42 | · | · |
| `bijection` → | · | · | (0) | · | 2 | · | · | · | · | · |
| `closure` → | · | · | · | (0) | · | · | · | · | · | · |
| `elim` → | · | · | · | · | (0) | · | · | · | · | · |
| `int` → | · | 93 | · | · | · | (62) | · | 18 | · | · |
| `num9` → | · | 1 | · | · | · | · | (137) | · | · | · |
| `rat` → | · | 61 | · | · | · | 11 | · | (54) | · | · |
| `slots` → | · | · | · | · | · | · | · | · | (0) | · |
| `typing` → | · | · | · | · | · | · | · | · | · | (0) |

**Imported together** (the number of files whose imports reach both modules, a module's own imports followed; the first column is the files that reach the module at all):

|  | files | `algebra` | `arith` | `bijection` | `closure` | `digits` | `elim` | `int` | `macro` | `num9` | `rat` | `slots` | `typing` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `algebra` | 95 | — | 1 | 13 | 3 | 4 | · | · | 95 | · | · | 10 | 2 |
| `arith` | 105 | 1 | — | · | · | 105 | · | 30 | 1 | 16 | 14 | · | · |
| `bijection` | 13 | 13 | · | — | 1 | · | · | · | 13 | · | · | · | · |
| `closure` | 3 | 3 | · | 1 | — | · | · | · | 3 | · | · | · | · |
| `digits` | 115 | 4 | 105 | · | · | — | · | 30 | 4 | 16 | 14 | · | · |
| `elim` | 5 | · | · | · | · | · | — | · | 5 | · | · | · | · |
| `int` | 30 | · | 30 | · | · | 30 | · | — | · | · | 14 | · | · |
| `macro` | 123 | 95 | 1 | 13 | 3 | 4 | 5 | · | — | · | · | 14 | 2 |
| `num9` | 16 | · | 16 | · | · | 16 | · | · | · | — | · | · | · |
| `rat` | 14 | · | 14 | · | · | 14 | · | 14 | · | · | — | · | · |
| `slots` | 14 | 10 | · | · | · | · | · | · | 14 | · | · | — | · |
| `typing` | 3 | 2 | · | · | · | · | · | · | 2 | · | · | · | — |

### The engine's strata beside the static graph's

`ein saturate FILE --dump` prints a stratum beside each rule and a `;; Strata` line, from the graph `ein_infer::strata` builds over the plans the run compiled. The script builds the same graph from the text: rules instantiated by the activators a file writes and by those a rule concludes with every argument a name, no fact matched.

- files of the census the command saturates and prints the line for: 409; saturated with no line, having no rule: 79; **refused at load or stopped, exit not 0: 144**, which hold 120 of the census's 1553 rules (103 distinct), and 71 shapes of level (ii) that no accepted file has. They are witnesses of a refusal that live outside `examples/broken/`, and the census reads them as written.
- the verdict (stratified or not) agrees on 409 of 409; the number of strata on 376.
- a rule's stratum agrees for 13653 of the 13962 rules the dumps list.

The first files on which the verdict or the count differs:

| file | the engine | from the text | rules' strata agreeing |
|---|---|---|---|
| `examples/domain_elim/ab.ein` | stratified, 2 strata | stratified, 1 | 5 of 11 |
| `examples/ein-bugs/wrong-answers/assert-eq-x-w.ein` | stratified, 1 stratum | stratified, 0 | 1 of 1 |
| `examples/ein-bugs/wrong-answers/assert-neq-w-x.ein` | stratified, 1 stratum | stratified, 0 | 1 of 1 |
| `examples/ein-bugs/wrong-answers/assert-neq-x-x.ein` | stratified, 1 stratum | stratified, 0 | 1 of 1 |
| `examples/ein-bugs/wrong-answers/forbid-chain-param-absent.ein` | stratified, 1 stratum | stratified, 2 | 1 of 2 |
| `examples/ein-bugs/wrong-answers/not-eq-assert.ein` | stratified, 1 stratum | stratified, 0 | 1 of 1 |
| `examples/ein-bugs/wrong-answers/relvar-eq-assert.ein` | stratified, 1 stratum | stratified, 0 | 1 of 1 |
| `examples/ein-bugs/zebra2-bad.ein` | stratified, 2 strata | stratified, 1 | 10 of 30 |


<!-- /generated: utils/rule_census.py -->

## What stands out

Observations on the run of 2026-11-12, each from a table above. No recommendation is made and no goal is
set: the user asked to see the results.

**The corpus.**

- 1 553 rules in 402 of 632 files, and 1 104 distinct ones: 449 rules are a rule some other file also
  writes, the zebra family and the witnesses under `examples/ein-bugs/` most of all.
  `examples/ein-bugs/` holds 465 rules, the stdlib 341, and the stdlib's four calculator modules 264 of
  those.
- Outside `examples/broken/`, 144 files are refused at load; they hold 120 rules and 71 level-(ii) shapes
  no accepted file has. A census of *programs that load* would be smaller by that much.

**Shapes.**

- 765 shapes at level (ii) for 1 553 rules. 101 shapes cover half the rules, 562 shapes are held by one
  rule each. Counted over distinct rules the picture is flatter: 213 shapes cover half of the 1 104.
- **The order of conjuncts carries almost nothing**: 770 shapes as written, 765 in any order. Telling
  constants from variables from parameters carries a little (708 at level (iii)). Which term sits where
  carries more (572 at level (iii′)). So most of the variety is in how many atoms there are, of which
  relations, and how they nest.
- **Most rules are small.** 810 rules (605 distinct) have one positive premise and nothing else in the
  `:match`; 898 are one premise and one conclusion; the median rule has one premise, one conclusion, two
  variables beside its parameters and no constant. The five counts of level (iv′) put 610 rules in one
  class.
- The most frequent shape, `(R1 ?a) ⇒ (R2 ?a)`, is 60 rules and 52 more as an hrule; 59 of the 60 and 51
  of the 52 are in `examples/`. The largest shapes are fixtures' and the copied stdlib rules'
  (`functional` in 23 files, `symmetric` and `transitive` in 16 each).
- **The stdlib's shapes are its own**: 222 of its 272 level-(ii) shapes occur in no other directory.
  `examples/beyond_arith` (117 of 128), `examples/word_problems` (45 of 48) and `examples/binary_logic`
  (47 of 53) are alike in that. A new theory has so far brought new shapes with it, not reused ones.
- The `:assert` alone has far fewer shapes (306) than the `:match` alone (492); six `:assert` shapes cover
  half the rules. 210 rules conclude `(false)` and nothing else.

**Families.**

- 88 families of distinct rules are one rule under a renaming of relations (347 rules); 16 are one rule
  under a renaming of constants (46 rules). Read slot for slot as written, 25 families vary in exactly one
  relation: `rat-add-ask` … `rat-div-ask` (5), `int-add-ask` … (4), the three `int-div-*-ask`.
- **The calculator is a few schemas repeated, and the repetition is of two sorts.** Per operation: the
  `*-ask`, `*-ask-second`, `*-answer` rules are one shape each across `add`, `sub`, `cmp`, `mul`, `div`,
  and the answer shape `(R1 (R2 ?a ?b) ?c) ⇒ (R3 ?a ?b ?c)` is 24 rules. Per representation:
  53 of the 68 rules of `examples/binary_arithmetic/bin.ein` are a rule of `stdlib/arith.ein` under a
  renaming of relations and nothing else, which is what its generator does. `std.int` and `std.rat` share
  13 and 14 such rules.
- **`div-try-1` to `div-try-9` are nine different shapes at levels (i), (ii) and (iii)** and one shape at
  (iii′). The digit is a constant in two places and also decides which of nine variables is read, so the
  nine differ in the pattern of variable sharing and no renaming of names maps one onto another. Level
  (iii′) finds 235 such families inside single files, 559 distinct rules.
- 4 families differ in exactly one constant as written, each of two rules.

**Groupings.**

- By number of relations, most rules have two (527) or three (415); by join structure, 947 rules have one
  positive premise, 241 a pair, 179 a product of premises that share no variable, 55 are cyclic.
- 495 rules nest: 178 take apart (the premises deeper than the conclusions), 116 build, and 201 are nested
  alike on both sides; 241 of the 495 are in the stdlib, the calculator's tags and numbers.
- **Of 1 429 `rule`s, 288 are exactly one of the 21 schemas the script recognises** (unary inclusion 81,
  the two *empty* forms 56, functional 25, default 20, symmetry 19, transitivity 17, inclusion 15), and
  1 141 are none of them.
- 179 rules conclude a relation one of their own premises reads; 30 of the 55 cyclic joins do.
- Of the levels, (iii) and (iii′) do not determine the number of variables or the join structure, and (iv)
  does not determine the arity profile: the skeleton and the relational pattern are two different
  coarsenings of level (ii), neither refining the other.

**Connections.**

- **Inside a file, almost half of all rule pairs share a name** (48 %), 35 % share a relation name and
  24 % a constant. Globally the same kinds connect 7 %, 4 % and 4 %: a name's spelling is reused across
  programs (`p`, `q`, `is-a`), which the global column measures and which means little.
- **Assert → match.** Inside files, 1 905 ordered pairs have a conclusion of A of a relation B reads, and
  1 480 of them unify: about a fifth of the links by name are ruled out by a constant, an arity or a
  nesting. A further 1 214 pairs link only through a rule with a variable at a head. Globally those generic
  links are 153 764, since a rule like `converse` can feed and be fed by every binary atom there is.
- **Negative and conflicting links are rare.** 46 pairs inside a file where a conclusion unifies with an
  atom tested by `absent`; 3 where a concluded negation is read; 5 where one rule concludes what another
  concludes the negation of. 113 pairs feed a rule that concludes `(false)`.
- 261 pairs of rules inside a file can derive the same fact (two conclusions unify); 1 347 can be fed by
  the same fact.
- **Activation.** Of 339 rules with parameters (112 distinct), 171 have their activator written as a fact
  somewhere in scope, 167 have it concluded by a rule, 23 are named in a `(solve :hrules …)` list and 6
  are switched on by nothing the script sees. 165 pairs inside a file are *a rule switching a rule on*.
- A closing rule's list (9d) and a rule naming a rule as a constant (1f) are the only places a rule's name
  is data today: 433 ordered pairs inside files (1f), the closing rules' lists most of them.

**The graph.**

- **Most files are flat.** Of the 402 files with a rule, 130 have any assert → match edge among their own
  rules and the median longest path is 0. The longest is 9, in `examples/word_problems/words.ein`.
- **`std.arith` is one knot**: 65 of its 89 rules are on a cycle and 54 are in one strongly connected
  component, its walks feeding one another through the tagged calls. `examples/binary_arithmetic/bin.ein`
  has a component of 33 of its 68.
- **`std.int` and `std.rat` are nearly acyclic alone** (62 and 54 edges, components of at most two rules,
  longest path 3) and tightly knit with what they are built on: 93 edges from `int` into `arith` and 86
  back, 61 from `rat` into `arith` and 42 back. Their cycles go through the lower module.
- **`std.algebra`, `std.bijection`, `std.elim`, `std.slots` and `std.typing` have almost no edge by
  name** (14 in `algebra`, none in the others): their rules are generic, every relation a parameter, and
  what connects them is the activators they conclude and the programs that import them. `std.num9` has one
  edge to another module, the division-by-zero flag.
- The two families of modules are never imported together but once (`algebra` and `arith`, one file).
- **The engine's strata and the static graph agree on the verdict for all 409 files** that saturate and
  hold a rule, 7 of them not stratifiable, on the number of strata for 376, and on a rule's stratum for
  13 653 of 13 962 rule listings. Where they differ, an activator is derived with a variable the script
  does not resolve (`examples/domain_elim/ab.ein`, the `zebra2` family).

## What was not computed

- **A shape's sub-patterns up to renaming.** 8k to 8m compare `:match`es with relations as written. Whether
  one *shape* is a sub-pattern of another with relations renamed too is a subgraph matching over 492
  `:match` shapes and was not run.
- **Shapes after macro expansion and after import.** A rule is read as written. The connections expand
  macros and the shapes do not.
- **Activators derived with variables.** The static strata instantiate a rule by the activators a file
  writes and by those a rule concludes with every argument a name. A setup rule that derives an activator
  from a matched fact (`std.slots`' fan-outs) is not followed, which is where the two graphs differ.
- **The loader's own import selection.** Program scope is the approximation stated above; a qualified
  import's renaming is ignored.
- **Exactness of the canonical labelling** past the tie limit is not proved; the first section's line says
  how many labellings took the fallback and that its two runs agree on all of them.
- **Whether a program loads** is known only from the engine run of the last section of § B, and the tables
  of § A and of the groupings do not separate the refused files' rules.
