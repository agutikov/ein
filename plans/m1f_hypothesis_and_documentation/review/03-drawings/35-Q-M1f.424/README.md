# Q-M1f.424 — where a drawing writes what a fact rests on

## 1. The question

A drawing of a knowledge base says, beside each fact, where it comes from: a given fact's source, shortened to its
id (`(10)` for `:source "condition (10)"`), and a derived fact's rule (`by symmetric`). Before S1f.23.1 the KB view
wrote this **inside the fact's octagon**, after the relation's name and a literal `\n` (L-96's stray backslash). With
the relation's name gone from the octagon, the build writes it **beside the octagon, as its `xlabel`**, and, on a
folded arrow, after the relation's name. A derived fact stays dashed (its octagon and its arrows), a fact a state
holds beyond `--since`'s is drawn at `penwidth=3`, and the colour stays by relation. The other reading: **a node of
its own for each source and each rule**, which the fact points at.

Recorded: [`open_questions.md` § Q-M1f.424](../../../open_questions.md#q-m1f424--where-what-a-fact-rests-on-is-drawn)
(put, not ruled; default (a)).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; every KB view drawn with `fdp`, the layout it suggests.

**The small program** ([`programs/small.ein`](programs/small.ein)), `ein render kb`
([`renders/small.kb.dot`](renders/small.kb.dot) / [`.svg`](renders/small.kb.svg)): `(owns Bob Dog)` has `(1)`
beside it (`:source "clue (1)"`), `(friends Bob Carol)` `(2)`; the three derived facts are dashed, with
`by symmetric` and `by pet-owner` (twice) beside them; a fact with no source, `(owns Carol Cat)`, a declaration or
`(symmetric friends)`, has no note. Each note is one line, in the fact's colour.

![ein render kb of the small program: (1) and (2) beside the two given facts' octagons, by symmetric and by pet-owner beside the dashed derived ones](renders/small.kb.svg)

**Folded** ([`renders/small.kb.fold.svg`](renders/small.kb.fold.svg), [dot](renders/small.kb.fold.dot)): the note
follows the relation's name on the arrow, `friends (2)`, `friends by symmetric`; a fact that stays an octagon (it is
pointed at, or has two or four arrows) keeps its note beside it, `(1)` on `(owns Bob Dog)`, which a fact of `claims`
points at.

![ein render kb --fold: friends (2) and friends by symmetric written on the two folded arrows between Bob and Carol; (1) beside the octagon of owns Bob Dog](renders/small.kb.fold.svg)

**Before S1f.23.1** (`build/review-bin/ein-head-b5fb8d70`, the same program,
[`renders/small.kb.before-b5fb8d70.svg`](renders/small.kb.before-b5fb8d70.svg),
[dot](renders/small.kb.before-b5fb8d70.dot)): the note was in the octagon's label, `(owns)\n(1)`, the `\n` drawn as
two characters; the folded arrows read `friends (2)`, `friends by symmetric` as today.

![before: the octagon labelled (owns)\n(1) with a literal backslash-n, ellipse atoms, the declarations as labelled octagons](renders/small.kb.before-b5fb8d70.svg)

**`--colour-by origin`** ([`renders/small.kb.origin.svg`](renders/small.kb.origin.svg),
[dot](renders/small.kb.origin.dot)): the notes are the same; the colour is black for a given fact with a source,
grey `#444444` for one with none (11 octagons, the declarations among them), blue and dashed for a derived one.

**`--since`**: [`programs/since.ein`](programs/since.ein) has an hrule asking whether Bob owns the dog and a rule
making an owner a pet owner; R is two states ([`runs/since.solve.txt`](runs/since.solve.txt)), dumped to
`runs/since.dump/`. `ein render kb runs/since.dump/models/02.ein --since runs/since.dump/models/01.ein`
([`renders/since.state2-since-state1.kb.svg`](renders/since.state2-since-state1.kb.svg),
[dot](renders/since.state2-since-state1.kb.dot)) draws at `penwidth=3` the two facts state 2 holds beyond state 1,
`(owns Bob Dog)` and `(pet-owner Bob)`, octagon and arrows; `(pet-owner Bob)` has `by pet-owner` beside it, and
`(owns Bob Dog)`, the commitment's own fact, **has no note**, dashed as a derived fact is (see *Found while
building*). Folded: [`renders/since.state2-since-state1.kb.fold.svg`](renders/since.state2-since-state1.kb.fold.svg).

![ein render kb of state 2 since state 1: the commitment's fact owns Bob Dog and the derived pet-owner Bob bold and dashed, by pet-owner beside the latter, (1) beside owns Carol Cat](renders/since.state2-since-state1.kb.svg)

**zebra2, folded** ([`programs/zebra2.ein`](programs/zebra2.ein), a copy of `examples/zebra2.ein`;
[`renders/zebra2.kb.fold.dot`](renders/zebra2.kb.fold.dot) / [`.svg`](renders/zebra2.kb.fold.svg), 1 042 arrows,
`fdp` 3.9 to 4.9 s): 363 arrows and octagons carry a note; the given facts' read `nation-loc (10)`,
`drink-loc (9)`, `right-of (1)` (four), the derived ones `is-a* by transitive` (66), `by co-located-fanout` (24),
`by std.bijection.functional-negative` (21), … ([`runs/notes.txt`](runs/notes.txt), every note of four views
counted). A rule of a module is named in full, `by std.bijection.functional-negative`; the program's own rules short,
`by co-located-negative`.

## 3. The options

### (a) Beside the octagon, an `xlabel` (the default, built)

As above. No node is added; a note is placed by Graphviz after the layout, near its octagon, and may sit over other
labels where a drawing is dense (zebra2).

### (b) A node of its own for the source or the rule, the fact pointing at it

Each source text and each rule one node, which every fact that rests on it points at, so the facts a clue gave, or a
rule derived, are gathered at one node. Needs a code change: **a mock**,
[`renders/small.kb.option-b.MOCK.dot`](renders/small.kb.option-b.MOCK.dot) /
[`.svg`](renders/small.kb.option-b.MOCK.svg): today's DOT of the small program with each fact's `xlabel` note deleted
and drawn instead as a `note`-shaped node (`clue (1)`, `clue (2)`, `by symmetric`, `by pet-owner`), the fact pointing
at it by a dotted arrow with no head and no number, since it is no arrow of the graph. Nothing else edited.

![MOCK, option (b): four note nodes, clue (1), clue (2), by symmetric and by pet-owner, each fact that rests on one joined to it by a dotted line](renders/small.kb.option-b.MOCK.svg)

Such a node is not an object of the knowledge base: a drawing that holds it is no longer the graph alone, which
Q-M1f.419's canonical form is.

## 4. The default

**(a)**, taken by S1f.23.1: beside the octagon, an `xlabel`; on a folded arrow after the relation's name, as before;
derived dashed, `--since`'s new facts at `penwidth=3`, colour by relation; the literal `\n` goes with the label, so
L-96 is gone from every view the stage rewrote. Built in `ein-render/src/fact_graph.rs` (`Look`), `kb_dot.rs`,
`ir_dot.rs`' `stated_look`; the derivation graph writes its source, rule or hypothesis there too.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | re-takes every run and drawing below |
| [`programs/small.ein`](programs/small.ein) | the small program, written for the drawings' review |
| [`programs/since.ein`](programs/since.ein) | two states, for `--since`, written for this item |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| [`renders/small.kb.dot`](renders/small.kb.dot) / [`.svg`](renders/small.kb.svg) | `ein render kb`, canonical |
| [`renders/small.kb.fold.dot`](renders/small.kb.fold.dot) / [`.svg`](renders/small.kb.fold.svg) | `ein render kb --fold` |
| [`renders/small.kb.origin.dot`](renders/small.kb.origin.dot) / [`.svg`](renders/small.kb.origin.svg) | `ein render kb --colour-by origin` |
| [`renders/small.kb.before-b5fb8d70.dot`](renders/small.kb.before-b5fb8d70.dot) / [`.svg`](renders/small.kb.before-b5fb8d70.svg) | `ein render kb` by the binary before S1f.23.1 |
| [`runs/since.solve.txt`](runs/since.solve.txt) | `ein solve programs/since.ein --dump-answer runs/since.dump` |
| `runs/since.dump/` | the dump: [`index.json`](runs/since.dump/index.json), [`models/01.ein`](runs/since.dump/models/01.ein), [`models/02.ein`](runs/since.dump/models/02.ein), … |
| [`renders/since.state2-since-state1.kb.dot`](renders/since.state2-since-state1.kb.dot) / [`.svg`](renders/since.state2-since-state1.kb.svg) | `ein render kb models/02.ein --since models/01.ein` |
| [`renders/since.state2-since-state1.kb.fold.dot`](renders/since.state2-since-state1.kb.fold.dot) / [`.svg`](renders/since.state2-since-state1.kb.fold.svg) | the same, `--fold` |
| [`renders/since.state2.kb.origin.dot`](renders/since.state2.kb.origin.dot) / [`.svg`](renders/since.state2.kb.origin.svg) | state 2, `--colour-by origin`: the commitment's fact blue and dashed, as a derived one |
| [`renders/zebra2.kb.fold.dot`](renders/zebra2.kb.fold.dot) / [`.svg`](renders/zebra2.kb.fold.svg) | `ein render kb --fold` of zebra2 |
| [`renders/small.kb.option-b.MOCK.dot`](renders/small.kb.option-b.MOCK.dot) / [`.svg`](renders/small.kb.option-b.MOCK.svg) | **mock**, option (b) |
| [`runs/notes.txt`](runs/notes.txt) | every note of four views, counted |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | every command and its exit code (all 0) |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds, a drawing |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/35-Q-M1f.424/rerun.sh
```

It runs today's binary (and `ein-head-b5fb8d70` for the one *before* drawing), dumps since.ein's answer, draws every
`.dot` with the layout it suggests, and draws the mock from its hand-edited DOT.

## 7. Found while building

- **A commitment's fact has no note, and is drawn as a derived fact.** In state 2 of since.ein, `(owns Bob Dog)`,
  the fact the commitment adds, is dashed and, under `--colour-by origin`, blue, as a fact a rule derived, with
  nothing beside it ([`renders/since.state2.kb.origin.dot`](renders/since.state2.kb.origin.dot)): the KB view's note
  is a source's id or `by <rule>`, and a hypothesis is neither (`kb_dot.rs`' `note`, read). The ledger says the
  derivation graph writes *its source, rule or hypothesis*; the KB view writes no hypothesis. A reading for the user:
  a note such as `committed`, or as is.
- **One note is one line.** Q-M1f.424's *as real lines where it has two parts* has no case here: no fact has both a
  source and a rule, and a folded arrow writes the relation and its note on one line, `friends by symmetric`.
- The colour of `claims` and `relation` is one (`#bcbd22`): the palette is picked by the relation's name, and two
  names share a colour.

## 8. Your call

- **(a)** beside the octagon, an `xlabel`; after the relation's name on a folded arrow (default, built).
- **(b)** a node of its own for each source and each rule, the fact pointing at it.
