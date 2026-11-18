# Q-M1f.425 — the fold's option, and what it folds

## 1. The question

You asked for the canonical drawing by default and, *optionally*, *"fold 3-facts into labelled arrows"*. The build
made the option one name, **`--fold`**, on every command that draws facts: `ein render program`, `kb`, `rules`,
`rule`, `constraints`, and `ein solve` for the trace's drawings; `--levi`, which asked for the octagons, is gone since
they are now the default. It folds a fact of **exactly three arrows** (a relation and two arguments) **that no drawn
fact points at** into one arrow from its first argument to its second, with the relation's name on it. It does not
fold a fact of two arrows into a loop on its argument (as the views drew one before), a fact of four arrows or more,
or a fact that another drawn fact points at, which must stay a node for that arrow to end at. The other reading: a
fact of two arrows folded too, into a loop.

Recorded: [`open_questions.md` § Q-M1f.425](../../../open_questions.md#q-m1f425--the-folds-option-and-what-it-folds)
(put, not ruled; default (a)).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`. Every command below ran on
[`programs/small.ein`](programs/small.ein) without and with `--fold`
([`runs/00-exit-codes.txt`](runs/00-exit-codes.txt): all exit 0); what each drawing holds, counted
([`runs/fold-counts.tsv`](runs/fold-counts.tsv): arrows, octagons, arrows with a word on them):

| command | canonical | `--fold` |
|---|---|---|
| `ein render kb` | 44 arrows, 16 octagons | 34 arrows, 11 octagons, 5 folded |
| `ein render program` (its `kb`, two rules, `query`) | 44 arrows, 15 octagons | 30 arrows, 8 octagons |
| `ein render rules` | 11 arrows, 4 octagons | 5 arrows, 1 octagon |
| `ein render rule --name symmetric` | 6 arrows, 2 octagons | 2 arrows, 0 octagons |
| `ein render constraints` | 27 arrows, 9 octagons | 23 arrows, 7 octagons, 2 folded |
| `ein solve --trace`, step 1's slice | 8 arrows, 2 octagons | 6 arrows, 0 octagons, 2 points |
| zebra2, `ein render kb` | 1 446 arrows, 452 octagons | 1 042 arrows, 250 octagons, 202 folded |
| add.ein, `ein render program` | 56 arrows, 18 octagons | 48 arrows, 14 octagons, 4 folded |

A command that draws no fact does not take it: `ein render lattice --fold` is refused, *unexpected argument
'--fold'*, exit 2 ([`runs/render-lattice.fold.txt.stderr.txt`](runs/render-lattice.fold.txt.stderr.txt)).

**The KB view** ([`renders/small.kb.svg`](renders/small.kb.svg) against
[`renders/small.kb.fold.svg`](renders/small.kb.fold.svg)): the five facts of three arrows that nothing points at are
arrows, `Carol → Cat` `owns`, `Bob → Carol` `friends (2)`, `Carol → Bob` `friends by symmetric`, and the two facts of
`claims`, each an arrow from the claimant to the octagon of what it claims. What stays an octagon, 11:

- **a fact of two arrows**, six: the loader's `(relation owns)` and `(relation friends)`, `(symmetric friends)`,
  `(pet-owner Bob)`, `(pet-owner Carol)`, and the stored negation `(not (owns Eve Dog))`;
- **a fact of four arrows**, two: the declarations `(relation owns Person Pet)`, `(relation friends Person Person)`;
- **a fact a drawn fact points at**, three: `(owns Bob Dog)`, held and claimed, `(owns Dan Fish)`, claimed only, in
  its frame `claims ctx`, and `(owns Eve Dog)`, which the negation points at, in `not ctx`.

![ein render kb --fold of the small program: owns, friends (2) and friends by symmetric as labelled arrows, the two claims as arrows into the octagons they claim; the facts of two and four arrows and the facts pointed at stay octagons](renders/small.kb.fold.svg)

**A rule's panels** ([`renders/small.rule-symmetric.fold.svg`](renders/small.rule-symmetric.fold.svg)): `symmetric`'s
`(?rel ?a ?b)` is folded too, an arrow `?a → ?b` labelled `?rel`, so the variable `?rel` is no longer a diamond but
the arrow's word (see *Found while building*); `pet-owner`'s `(owns ?p ?x)` an arrow `owns`, its `(pet-owner ?p)`, of
two arrows, an octagon ([`renders/small.rules.fold.02-rule_pet_owner_lhs_rhs.svg`](renders/small.rules.fold.02-rule_pet_owner_lhs_rhs.svg)).

![ein render rule --name symmetric --fold: in match an arrow ?a to ?b labelled ?rel, in assert a dashed arrow ?b to ?a labelled ?rel; no octagon, no diamond for ?rel](renders/small.rule-symmetric.fold.svg)

**A trace's slice** ([`runs/small.trace.fold.md`](runs/small.trace.fold.md), its slices cut to
`renders/small.trace.fold.root.step*.dot`): the premise `(owns Bob Dog)` is an arrow from `Bob` to a point,
labelled `owns`, and on to `Dog`; the firing's arrow leaves the point (how a slice folds is
[Q-M1f.429](../40-Q-M1f.429/README.md)'s).

![step 2 of the trace under --fold: the premise owns Bob Dog an arrow through a point, the rule pet-owner from the point to the octagon of pet-owner Bob, which has two arrows and stays an octagon](renders/small.trace.fold.root.step2.svg)

**Real programs.** zebra2's KB ([`programs/zebra2.ein`](programs/zebra2.ein), a copy of `examples/zebra2.ein`):
[`renders/zebra2.kb.svg`](renders/zebra2.kb.svg) (`fdp` 19.9 s) and
[`renders/zebra2.kb.fold.svg`](renders/zebra2.kb.fold.svg) (`fdp` 3.9 s): 202 facts fold and 250 stay octagons: 101 of two arrows (62 stored negations, 9 `(relation r)`,
the facts that apply a module's rule to a relation), 87 of four to six arrows (the declarations, the `co-located` and
`adjacent-via` facts), and 62 of three arrows that a negation points at. The calculator's
program view ([`programs/add.ein`](programs/add.ein), a copy of `examples/arith/add.ein`;
[`renders/add.program.01-kb.svg`](renders/add.program.01-kb.svg) against
[`renders/add.program.fold.01-kb.svg`](renders/add.program.fold.01-kb.svg)): four facts fold, `(add apples pears)`,
the two `let`s (each an arrow from the name to its numeral's octagon) and the first request, an arrow `add` between
its operands' octagons; every numeral `(n … d)` stays an octagon, since the fact above points at it.

![ein render program --fold of add.ein, the digraph kb: apples to pears labelled add, the lets as arrows into numeral octagons, the numerals a knot of octagons around the box n](renders/add.program.fold.01-kb.svg)

## 3. The options

### (a) `--fold` on every command that draws facts; a fact of exactly three arrows that nothing points at (the default, built)

As above.

### (b) A fact of two arrows folded too, into a loop on its argument

As the KB view drew one before S1f.23.1 (`"owns" -> "owns" [label="relation"]`). Needs a code change: **a mock**,
[`renders/small.kb.fold.option-b-loops.MOCK.dot`](renders/small.kb.fold.option-b-loops.MOCK.dot) /
[`.svg`](renders/small.kb.fold.option-b-loops.MOCK.svg): today's `--fold` DOT with each fact of two arrows that nothing
points at (its octagon and its two arrows) deleted and drawn as a loop on its argument, its relation's name and note
the loop's `xlabel`. Six loops: `relation` on `owns` and on `friends`, `symmetric` on `friends`, `pet-owner by
pet-owner` on `Bob` and on `Carol`, and `not` on the octagon of `(owns Eve Dog)`, a loop on a fact.

![MOCK, option (b): the six facts of two arrows as loops: relation and symmetric looping on friends, pet-owner on Bob and on Carol, not on the octagon of owns Eve Dog](renders/small.kb.fold.option-b-loops.MOCK.svg)

A loop draws the fact of two arrows as a mark on one atom, and two such facts on one atom (`friends` above) as two
loops; a negation's loop sits on the fact it denies.

## 4. The default

**(a)**, taken by S1f.23.1: `--fold`, one name on every subcommand that draws facts; `--levi` goes, since the
octagon is the default; it folds a fact of exactly three arrows that no drawn fact points at; a fact of two arrows is
no loop, and a fact a drawn fact points at is a node. Built in `ein-cli/src/cmdline.rs`' `fold`; held by
`every_object_once.rs` and `render_doors.rs`' `every_door_draws_a_fact_in_both_forms`; the pages `12-rendering.md`,
`05-cli.md`, `06-config.md`.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | re-takes every run and drawing below |
| [`programs/small.ein`](programs/small.ein) | the small program, written for the drawings' review |
| [`programs/zebra2.ein`](programs/zebra2.ein), [`programs/add.ein`](programs/add.ein) | copies of `examples/zebra2.ein` and `examples/arith/add.ein` |
| [`renders/small.kb.dot`](renders/small.kb.dot) / [`.svg`](renders/small.kb.svg), [`renders/small.kb.fold.dot`](renders/small.kb.fold.dot) / [`.svg`](renders/small.kb.fold.svg) | `ein render kb`, without and with `--fold` |
| [`renders/small.program.dot`](renders/small.program.dot), [`renders/small.program.fold.dot`](renders/small.program.fold.dot) | `ein render program`, four digraphs each, split into `small.program[.fold].01-kb`, `.02-rule_symmetric_lhs_rhs`, `.03-rule_pet_owner_lhs_rhs`, `.04-query` (`.dot`, `.svg`): [kb](renders/small.program.01-kb.svg), [kb folded](renders/small.program.fold.01-kb.svg) |
| [`renders/small.rules.dot`](renders/small.rules.dot), [`renders/small.rules.fold.dot`](renders/small.rules.fold.dot) | `ein render rules`, split into `.01-rule_symmetric_lhs_rhs` and `.02-rule_pet_owner_lhs_rhs`: [folded, pet-owner](renders/small.rules.fold.02-rule_pet_owner_lhs_rhs.svg) |
| [`renders/small.rule-symmetric.dot`](renders/small.rule-symmetric.dot) / [`.svg`](renders/small.rule-symmetric.svg), [`renders/small.rule-symmetric.fold.dot`](renders/small.rule-symmetric.fold.dot) / [`.svg`](renders/small.rule-symmetric.fold.svg) | `ein render rule --name symmetric` |
| [`renders/small.constraints.dot`](renders/small.constraints.dot) / [`.svg`](renders/small.constraints.svg), [`renders/small.constraints.fold.dot`](renders/small.constraints.fold.dot) / [`.svg`](renders/small.constraints.fold.svg) | `ein render constraints` |
| [`runs/small.trace.md`](runs/small.trace.md), [`runs/small.trace.fold.md`](runs/small.trace.fold.md) | `ein solve --trace`, without and with `--fold` (their read-outs `runs/small.trace[.fold].stdout.txt`) |
| `renders/small.trace.root.step1.dot` … `step4`, `renders/small.trace.fold.root.step1.dot` … `step4` (and `.svg`) | the slices cut from the two traces: [step 1](renders/small.trace.root.step1.svg), [step 1 folded](renders/small.trace.fold.root.step1.svg) |
| [`renders/zebra2.kb.dot`](renders/zebra2.kb.dot) / [`.svg`](renders/zebra2.kb.svg), [`renders/zebra2.kb.fold.dot`](renders/zebra2.kb.fold.dot) / [`.svg`](renders/zebra2.kb.fold.svg) | zebra2's KB view |
| [`renders/add.program.dot`](renders/add.program.dot), [`renders/add.program.fold.dot`](renders/add.program.fold.dot) | add.ein's program view, split into `.01-kb` and `.02-query`: [kb](renders/add.program.01-kb.svg), [kb folded](renders/add.program.fold.01-kb.svg) |
| [`renders/small.kb.fold.option-b-loops.MOCK.dot`](renders/small.kb.fold.option-b-loops.MOCK.dot) / [`.svg`](renders/small.kb.fold.option-b-loops.MOCK.svg) | **mock**, option (b) |
| [`runs/render-lattice.fold.txt.stderr.txt`](runs/render-lattice.fold.txt.stderr.txt) | `ein render lattice --fold`, refused |
| [`runs/fold-counts.tsv`](runs/fold-counts.tsv) | arrows, octagons and words on arrows, per drawing |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt), [`runs/draw-times.tsv`](runs/draw-times.tsv) | every command's exit code; Graphviz's seconds a drawing |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/36-Q-M1f.425/rerun.sh
```

## 7. Found while building

- **A pattern whose relation is a variable folds, and its variable becomes a word.** Under `--fold`, `symmetric`'s
  `(?rel ?a ?b)` is the arrow `?a → ?b` labelled `?rel`: the diamond `?rel` is gone from both panels, so where the
  same variable is also an argument of another pattern (std.bijection's `(relation ?R ?A ?B)` beside `(?R ?a ?b)`),
  one variable is drawn as a diamond in one place and as a label in another. The ruling folds *a fact of three
  arrows*, and is silent on a variable in the relation's place.
- **A fact of three arrows whose argument is a fact folds**, into an arrow that ends at that fact's octagon:
  `(claims Carol (owns Bob Dog))` is an arrow `Carol → ⬠` labelled `claims`, and add.ein's first request an arrow
  between two numerals' octagons. As the ruling reads; shown here since no page draws one.
- zebra2's whole KB, canonical, took `fdp` 19.9 s here, folded 3.9 s.

## 8. Your call

- **(a)** `--fold` on every command that draws facts; it folds a fact of exactly three arrows that nothing points at
  (default, built).
- **(b)** a fact of two arrows folded too, into a loop on its argument.
