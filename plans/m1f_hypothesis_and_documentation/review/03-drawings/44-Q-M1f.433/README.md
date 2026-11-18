# Q-M1f.433 — every atom a box

## 1. The question

The user ruled that `is-a` and `relation` are ordinary relations and that no drawing gives either a look of its own
(Q-M1f.421): before, the KB view drew an atom an `is-a` fact pointed at as a box, one an `is-a` fact started from as
an oval, and every other atom as Graphviz's default ellipse. *Every atom one shape* leaves open which shape. The
build took **a box**, as the reasoning pages draw an atom: the node default of the KB, program and constraint views,
and `box` for an atom in a rule's panel, where a variable is a diamond. The other reading, (b), is Graphviz's
default **ellipse**, as the KB view drew an atom that was neither a type nor an instance.

Recorded: [`open_questions.md` § Q-M1f.433](../../../open_questions.md#q-m1f433--the-shape-of-an-atom); the ruling
it reads, [§ Q-M1f.421](../../../open_questions.md#q-m1f421--is-a-and-relation-in-a-drawing).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; *before* is `build/review-bin/ein-head-b5fb8d70`, the binary
before the stage. Programs: [`programs/small.ein`](programs/small.ein), the drawings' shared small program, and
[`programs/zebra2.ein`](programs/zebra2.ein), a copy of `examples/zebra2.ein`. Each view in both forms; KB and
program views drawn with `fdp` (their DOT suggests it), the others with `dot`.

**The shapes each view writes**, counted ([`runs/shapes.tsv`](runs/shapes.tsv)):

| view | node default | shapes set on a node |
|---|---|---|
| `kb` (small, zebra2) | `box` | `octagon` alone (16; 452) |
| `program` (small, zebra2) | `box` | `octagon`, `diamond` (a variable in a rule or the query), `box` (an atom in a rule's panel), `note` |
| `constraints` (small, zebra2) | `box` | `octagon` alone |
| `rules` (small, zebra2) | Graphviz's, unset | `octagon`, `diamond`, `box` (every atom: 2 in small's, 33 in zebra2's) |
| `kb` **before** (zebra2) | unset: ellipse | `box` 8 (types), `oval` 30 (instances), `octagon` 13 (labelled facts) |
| `constraints` **before** (small, zebra2) | unset: ellipse | `rectangle` (7; 11), `octagon` |

So today an atom is a box in every view that draws facts; a fact is an octagon, a variable a diamond, a query's
clauses a note. Before, `small.ein`, which writes no `is-a`, had every atom an ellipse
([`renders/small.kb.before-b5fb8d70.svg`](renders/small.kb.before-b5fb8d70.svg)).

![ein render kb --fold of small.ein: every atom a box, Bob, Carol, owns, relation, not and the rest; the facts that stay octagons, the declarations, the claim's inner facts, the negation, the fact of symmetric, the derived facts of pet-owner](renders/small.kb.fold.svg)

([dot](renders/small.kb.fold.dot); canonical, [`renders/small.kb.svg`](renders/small.kb.svg).) A rule's panel,
atoms boxes and variables diamonds:
[`renders/small.rules.02-rule_pet_owner_lhs_rhs.svg`](renders/small.rules.02-rule_pet_owner_lhs_rhs.svg); the
constraint view, [`renders/small.constraints.svg`](renders/small.constraints.svg); the program view's `kb`,
[`renders/small.program.01-kb.svg`](renders/small.program.01-kb.svg).

**Before**, on zebra2 ([`renders/zebra2.kb.before-b5fb8d70.svg`](renders/zebra2.kb.before-b5fb8d70.svg)): the boxes
`House`, `Color`, `Nationality`, `Pet`, `Drink`, `Cigarette`, `Attribute`, `T` (each the second argument of an
`is-a`), ovals for `Red`, `Norwegian`, `House-1` and the other instances, ellipses for the relations; each declaration
a labelled octagon `(relation)` and a fact of one argument a loop.

![before: ein-head-b5fb8d70's ein render kb of zebra2, fdp: eight boxes for the types, ovals for the instances and the relations, dashed is-a arrows with empty heads, labelled octagons (relation) and (adjacent-via)\n(6) with a literal \n, loops labelled relation and bijective](renders/zebra2.kb.before-b5fb8d70.svg)

## 3. The options

### (a) A box (the default)

§ 2: the reasoning pages' atom (`01-canonical-graph.dot`), one shape in every view.

### (b) Graphviz's default ellipse

**Mocks**: today's DOT with the node default's `shape=box` taken out, so an atom is an ellipse; one comment line
added, nothing else edited:
[`renders/small.kb.ellipse.MOCK.dot`](renders/small.kb.ellipse.MOCK.dot) / [`.svg`](renders/small.kb.ellipse.MOCK.svg),
[`renders/small.kb.fold.ellipse.MOCK.dot`](renders/small.kb.fold.ellipse.MOCK.dot) / [`.svg`](renders/small.kb.fold.ellipse.MOCK.svg),
[`renders/small.constraints.ellipse.MOCK.dot`](renders/small.constraints.ellipse.MOCK.dot) / [`.svg`](renders/small.constraints.ellipse.MOCK.svg),
[`renders/zebra2.kb.fold.ellipse.MOCK.dot`](renders/zebra2.kb.fold.ellipse.MOCK.dot) / [`.svg`](renders/zebra2.kb.fold.ellipse.MOCK.svg).
A rule's panel sets `shape=box` on each atom, so (b) there is the same edit on each atom's line.

![Mock of option (b): the same folded knowledge base of small.ein with every atom an ellipse; octagons as before](renders/small.kb.fold.ellipse.MOCK.svg)

The octagon and the ellipse are told apart as easily as the octagon and the box; the ellipse is wider for a long
name. Either way one shape for every atom.

## 4. The default

**(a), a box** (the ledger): *a box, as the reasoning pages draw one: the node default of the KB, program and
constraint views, so no atom has a line of its own, and `box` in a rule's panel, where a variable is a diamond.*
Built: `kb_dot.rs`' `kb_digraph_open`, `dot_util.rs`' `GROUND_SHAPE`.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every drawing below, from the repo root |
| [`programs/small.ein`](programs/small.ein) | the drawings' shared small program |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| `renders/NAME.VIEW.dot` / `.svg`, VIEW `kb`, `program`, `constraints`, `rules` ([small kb](renders/small.kb.svg), [small constraints](renders/small.constraints.svg), [zebra2 kb](renders/zebra2.kb.svg), [zebra2 constraints](renders/zebra2.constraints.svg), [small program](renders/small.program.dot), [small rules](renders/small.rules.dot), [zebra2 program](renders/zebra2.program.dot), [zebra2 rules](renders/zebra2.rules.dot)) | today, canonical; a program or rules file is drawn as its parts, `NAME.VIEW.NN-DIGRAPH.svg` |
| `renders/NAME.VIEW.fold.dot` / `.svg` ([small kb](renders/small.kb.fold.svg), [zebra2 kb](renders/zebra2.kb.fold.svg), [small constraints](renders/small.constraints.fold.svg), [zebra2 constraints](renders/zebra2.constraints.fold.svg)) | today, `--fold` |
| `renders/NAME.kb.before-b5fb8d70.dot` / `.svg`, `renders/NAME.constraints.before-b5fb8d70.dot` / `.svg` ([small kb](renders/small.kb.before-b5fb8d70.svg), [zebra2 kb](renders/zebra2.kb.before-b5fb8d70.svg), [small constraints](renders/small.constraints.before-b5fb8d70.svg), [zebra2 constraints](renders/zebra2.constraints.before-b5fb8d70.svg)) | before the stage |
| `renders/*.ellipse.MOCK.dot` / `.svg` (four, listed in § 3) | option (b), mocks |
| [`runs/shapes.tsv`](runs/shapes.tsv) | each drawing's node default and the shapes set on its nodes |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | each drawing's layout and Graphviz's seconds |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | each command's exit code |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/44-Q-M1f.433/rerun.sh
```

It runs `ein render kb|program|constraints|rules` (both forms) with `build/review-bin/ein-head-17085b27` and
`ein render kb|constraints` with `build/review-bin/ein-head-b5fb8d70`, counts the shapes, and draws every `.dot`,
the mocks' from their kept sources.

## 7. Found while building

- **A rule's view sets no node default**: each atom carries `shape=box` on its own line, where the KB, program and
  constraint views write `node [… shape=box]` once. Both draw a box; a reader editing the DOT by hand meets two ways.
- The h-space view's `exclusion` graph draws a **candidate** (a fact) as a box (item 41): outside the drawings of
  facts, a box is not always an atom.

## 8. Your call

- **(a) A box** (default), in every view that draws facts.
- **(b) An ellipse**, Graphviz's default.
