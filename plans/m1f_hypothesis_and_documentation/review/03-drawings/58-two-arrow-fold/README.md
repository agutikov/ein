# 58 · The fold of a fact of two arrows, as built

[← the topic](../README.md) · [← the bundle](../../README.md)

## The question

You ruled the fold (Q-M1f.425) *"(a) + (b)"*: under `--fold`, a fact of three arrows that nothing drawn points at is
one labelled arrow, and **a fact of two arrows that nothing drawn points at is a loop on its argument**, labelled with
its relation. S1f.23.6 built it in every view that draws facts and in a rule's panels. Two consequences the ruling did
not name are shown here for your look (S1f.23.6's Record, readings 4 and 5): **a stored negation `(not g)` is a fact
of two arrows**, so folded it is a loop labelled `not` on `g`'s octagon; and **a closing rule of the standard
library**, whose conclusion is a list of `(__closed__ R)`, each of two arrows, is now small enough folded to be drawn
on its page. Recorded in [`open_questions.md`](../../../open_questions.md#q-m1f425--the-folds-option-and-what-it-folds)
and the decision [Q-M1f.425](../../../../../docs/decisions/cli/Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md).

## What is so today

*Before* is the binary at the stage's start, *after* this stage's; each `runs/NAME.diff` is the DOT's change.

- **The rendering page's program** ([`rendering-page.ein`](programs/rendering-page.ein)): `ein render kb --fold`
  draws `(symmetric co-located)` and the two `(relation r)` facts the loader adds as loops, three loops and two
  octagons where there were five octagons ([after](renders/page-program.kb-fold.after.svg),
  [before](renders/page-program.kb-fold.before.svg)); the program view's `kb` has one loop, `symmetric`
  ([after](renders/page-program.program-kb-fold.after.svg)).
- **Stored negations** ([`02_elimination.ein`](programs/02_elimination.ein), the bijections page's program,
  `ein render kb --fold`): each `(not (wears X C))` is a loop labelled `not` on the octagon of the `wears` fact it
  denies, which stands in the frame `not ctx`; fourteen loops and eight octagons where there were twenty-two
  octagons ([after](renders/elimination.kb-fold.after.svg), [before](renders/elimination.kb-fold.before.svg)). The
  page `04-bijections.md` shows the same, cut.
- **A rule's pattern of two arrows** ([`binding-key-both.ein`](programs/binding-key-both.ein), the rule `note`,
  `ein render rule --fold`): `(holds ?f)` is a loop on `?f` ([after](renders/note.rule-fold.after.svg),
  [before](renders/note.rule-fold.before.svg)).
- **A closing rule** (`stdlib/math/decimal-arith.ein`, the rule `decimal-arith`, `ein render rule --fold`): 114
  loops and one octagon where there were 115 octagons ([after](renders/decimal-arith.rule-fold.after.svg)). The
  generator now draws five closing rules folded (`decimal-arith`, `rat-arith`, `binary-arith`, `arith9`, `props`)
  that were shown as Ein alone; `int-arith`'s is still past the bound. Of 511 rules: 493 canonical, 6 folded, 12 Ein
  only (before: 493, 1, 17).

## The options

- **(a) As built** (default): every fact of two arrows that nothing drawn points at folds, a stored negation and a
  closing rule's `(__closed__ R)` among them.
- **(b) A stored negation is never folded**: `not` read by its name, its octagon kept, so a denial reads as a fact
  pointing at the fact it denies. (It reads one name, `not`, the kernel's own.)
- **(c) A closing rule stays Ein only** on the reference pages, as before, read by its role (it names every relation
  of its module).

## The default

(a): the ruling folds a fact of two arrows, and a negation is one; no name is read.

## Files

- `programs/rendering-page.ein` (12-rendering.md's program, as the page states it), `programs/02_elimination.ein`
  (04-bijections.md's), `programs/binding-key-both.ein` (from `examples/ein-bugs/`); `decimal-arith` is read from
  `stdlib/` as the generator reads it.
- `renders/NAME.before|after.dot|svg`, `runs/NAME.diff`, `runs/NAME.*.layers.txt`.

## Re-run

`bash plans/m1f_hypothesis_and_documentation/review/03-drawings/tools/build.sh` from the repo root.

## Your call

- (a) as built.
- (b) a stored negation never folded.
- (c) a closing rule shown as Ein alone.
