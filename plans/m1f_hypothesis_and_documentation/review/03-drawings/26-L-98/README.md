# L-98 — the program view is several digraphs, so one fact can be drawn twice

## 1. The question

`ein render program FILE` draws the program as written, **one digraph per group**: `ontology` for the background
facts (no `:source`) and the declarations, `facts` for the given ones (with a `:source`), `query`, `solve`, and one
per rule. The ruled convention is that **every object is drawn once** (Q-M1f.395), but each digraph is a drawing of
its own, so the rule holds in each digraph and not across them. A fact stated in one group and pointed at from
another is therefore drawn twice: as an arrow in the first digraph (nothing points at it there) and as an octagon in
the second (a fact points at it there). Atoms written in two groups are drawn in each too. Should the program view
become **one digraph**?

Recorded: [`../../findings.md`](../../../findings.md#the-engines-renderers), row **L-98** (P1f.22's N-27). The row's
last sentence, a negation drawn as the fact it denies, is shown too (§ 2, end).

## 2. What is so today

All runs use today's binary, `build/review-bin/ein-head-b5fb8d70`.

**The witness** (the row's): [`programs/claims.ein`](programs/claims.ein).

```lisp
(owns Bob Dog)
(claims Carol (owns Bob Dog) :source "c")
(solve :no-search)
```

`ein render program programs/claims.ein` → [`runs/claims.render-program.dot`](runs/claims.render-program.dot), three
digraphs. Each is cut to its own file and drawn:

<!-- output -->
```text
digraph ontology {
  "Bob" [shape=rectangle];
  "Dog" [shape=rectangle];
  "Bob" -> "Dog" [label="owns", color="#ff7f0e", fontcolor="#ff7f0e", style=solid];
}

digraph facts {
  ranksep=0.55;
  "Carol" [shape=rectangle];
  "(owns Bob Dog)" [shape=octagon, label="owns"];
  "Bob" [shape=rectangle];
  "Dog" [shape=rectangle];
  "Carol" -> "(owns Bob Dog)" [label="claims", color="#bcbd22", fontcolor="#bcbd22", style=solid];
  "(owns Bob Dog)" -> "Bob" [label="1"];
  "(owns Bob Dog)" -> "Dog" [label="2"];
}

digraph solve {
  "solve" [shape=note, label=":no-search (#empty)"];
}
```

| digraph | DOT | picture |
|---|---|---|
| 1 `ontology` | [`renders/claims.render-program.1-ontology.dot`](renders/claims.render-program.1-ontology.dot) | [`.svg`](renders/claims.render-program.1-ontology.svg) |
| 2 `facts` | [`renders/claims.render-program.2-facts.dot`](renders/claims.render-program.2-facts.dot) | [`.svg`](renders/claims.render-program.2-facts.svg) |
| 3 `solve` | [`renders/claims.render-program.3-solve.dot`](renders/claims.render-program.3-solve.dot) | [`.svg`](renders/claims.render-program.3-solve.svg) |

![digraph ontology: (owns Bob Dog) as an arrow Bob to Dog labelled owns](renders/claims.render-program.1-ontology.svg)
![digraph facts: the same fact (owns Bob Dog) as an octagon owns, with claims entering it from Carol](renders/claims.render-program.2-facts.svg)

**The fact drawn in two digraphs is `(owns Bob Dog)`**: in `ontology` it is the orange arrow `Bob → Dog` labelled
`owns`; in `facts` it is the octagon `owns` with arrows 1 and 2 to `Bob` and `Dog`, which `claims` enters. `Bob` and
`Dog` are drawn in both. In `facts` the octagon stands in no frame: the program states the fact (in the other
group), so it is held, as Q-M1f.398 says — the frame rule already reads across groups; the drawing does not.

- `--levi` ([`runs/claims.render-program-levi.dot`](runs/claims.render-program-levi.dot), cut and drawn as
  `renders/claims.render-program-levi.{1-ontology,2-facts,3-solve}.{dot,svg}`): the fact is then **two octagons**,
  `h_1_owns` in `ontology` and `(owns Bob Dog)` in `facts`, with different node ids.
- Side by side, the two digraphs today, the two mocks of § 3, and the KB view:
  [`renders/claims.side-by-side.png`](renders/claims.side-by-side.png).
- For comparison, `ein render kb` (one digraph by construction) draws the fact once, an octagon in no frame:
  [`renders/claims.kb.dot`](renders/claims.kb.dot) / [`.svg`](renders/claims.kb.svg) (drawn with `fdp`).

**The rendering page's own example, with sources.** `12-rendering.md` § A fact under an arrow shows the program view
of four facts as one digraph, because all four are background facts. With a `:source` on the three that point at
another ([`programs/page-example-sourced.ein`](programs/page-example-sourced.ein), written for this review), the
same facts split: `ontology` draws `(owns Bob Dog)` as an arrow, `facts` as an octagon entered by `claims` and
`doubts`, and `(owns Carol Cat)` in its frame `claims ctx`
([`runs/page-example-sourced.render-program.dot`](runs/page-example-sourced.render-program.dot);
[`1-ontology.svg`](renders/page-example-sourced.render-program.1-ontology.svg),
[`2-facts.svg`](renders/page-example-sourced.render-program.2-facts.svg),
[`3-solve.svg`](renders/page-example-sourced.render-program.3-solve.svg); the KB view,
[`renders/page-example-sourced.kb.svg`](renders/page-example-sourced.kb.svg)).

**In the corpus.** `ein render program` on every corpus program (669 draw; the 225 refused are the 224 refused at
load plus `examples/syntax/config.ein`, *no renderable forms*:
[`runs/corpus-sweep.render-program.refused.tsv`](runs/corpus-sweep.render-program.refused.tsv)):

- 75 programs draw both an `ontology` and a `facts` digraph;
- **34** draw some atom in two of the fact digraphs, 373 atoms in all; `examples/zebra2.ein` draws 35 of its atoms
  twice ([`runs/corpus-sweep.render-program.atoms-in-two-digraphs.tsv`](runs/corpus-sweep.render-program.atoms-in-two-digraphs.tsv));
- **no** corpus program has the row's case, a fact stated in one group and pointed at from another. Two draw one
  fact in two digraphs because they write it twice, once in each group: `examples/syntax/negation.ein`,
  `(not (r a b))` and `(not (r a b) :source "(9)")`; `examples/syntax/unary-and-nary.ein`, `(u a)` and
  `(u a :source "(1)")`
  ([`runs/corpus-sweep.render-program.facts-in-two-digraphs.tsv`](runs/corpus-sweep.render-program.facts-in-two-digraphs.tsv);
  a heuristic over the DOT: an octagon `"(rel …)"` or an arrow labelled with a relation, in two non-rule digraphs).

**The negation, the row's last sentence.** [`programs/not1.ein`](programs/not1.ein), `(not (owns Bob Rex))`, and
[`programs/not2.ein`](programs/not2.ein), `(not (not (owns Bob Rex)))`, give the same bytes
([`runs/not1-vs-not2.cmp.txt`](runs/not1-vs-not2.cmp.txt)): one dashed arrow `Bob → Rex` labelled `owns`, the drawing
of the fact denied ([`renders/not1-and-not2.render-program.1-ontology.svg`](renders/not1-and-not2.render-program.1-ontology.svg)).
No option below changes that; it is the row's, and named here only so that it is not lost.

The library's view of one form alone (`to_dot_form`) knows no other form: read, not run. Why the program view
splits at all, in the code's own words (`ein-render/src/ir_dot.rs`, `render_group`, read): *"Render-bucket names,
not knowledge layers: the grouping is there only because three sub-graphs read better than one."* A form goes to
`reasoning` if it has `:rule` or `:using`, to `facts` if it has `:source`, else to `ontology`.

## 3. The options

### One digraph

Every group's forms drawn in one digraph, each object declared once: an atom once, and a fact that something
points at as its octagon only, never also as an arrow. **Mocks**, hand-merged from the binary's DOT:

- [`renders/claims.program.one-digraph.MOCK.dot`](renders/claims.program.one-digraph.MOCK.dot) /
  [`.svg`](renders/claims.program.one-digraph.MOCK.svg): `ontology`, `facts` and `solve` made one digraph;
  `ontology`'s arrow `Bob → Dog` dropped, `facts`' octagon kept; every remaining line is the binary's.

  ![mock: one digraph, (owns Bob Dog) once as its octagon, claims entering it from Carol, the solve note beside](renders/claims.program.one-digraph.MOCK.svg)

  What is lost is which group each form was written in: here, that `(owns Bob Dog)` is a background fact and the
  claim a given one. The digraph names (`ontology`, `facts`) are the only place the view says it today.

- [`renders/claims.program.one-digraph-origin.MOCK.dot`](renders/claims.program.one-digraph-origin.MOCK.dot) /
  [`.svg`](renders/claims.program.one-digraph-origin.MOCK.svg): the same, with the group kept as a colour the way
  `ein render kb --colour-by origin` already writes it (background grey, given black, the given fact's label with
  its short source, `claims c`).

  ![mock: one digraph coloured by origin, the background fact grey, the given claim black and labelled claims c](renders/claims.program.one-digraph-origin.MOCK.svg)

- [`renders/page-example-sourced.program.one-digraph.MOCK.dot`](renders/page-example-sourced.program.one-digraph.MOCK.dot) /
  [`.svg`](renders/page-example-sourced.program.one-digraph.MOCK.svg): the page example merged; it is the `facts`
  digraph as the binary wrote it plus the solve note, and is the drawing the rendering page shows for the unsourced
  program.

What else it changes (read, not run): every program's program-view lines of the shape golden (`dot[ir]`,
`dot[ir-levi]`, `dot[ir-overlay]`, … one per corpus program each); the rendering page's two program-view drawings,
`12-program-ontology` and `12-program-facts`, which show one group each, and its words *one digraph per group*
(also in `05-cli.md`); and whether rules and the query join the one digraph or stay their own (the mocks put the
solve note in).

### As is

One digraph per group; every object drawn once **within** a digraph. The row's case is rare in the corpus (none
today); atoms drawn in two digraphs are common where a program has given facts (34 programs).

## 4. The default

The row names no default: **the user's call**, *"whether the program view becomes one digraph"*. Until ruled, the
view stays as it is.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`programs/claims.ein`](programs/claims.ein) | the row's witness, written for this review |
| [`programs/page-example-sourced.ein`](programs/page-example-sourced.ein) | the rendering page's example with `:source` on three facts |
| [`programs/not1.ein`](programs/not1.ein), [`programs/not2.ein`](programs/not2.ein) | `(not g)` and `(not (not g))` |
| [`runs/claims.render-program.dot`](runs/claims.render-program.dot) | `ein render program`, raw, three digraphs |
| [`runs/claims.render-program-levi.dot`](runs/claims.render-program-levi.dot) | `--levi`, raw |
| [`runs/page-example-sourced.render-program.dot`](runs/page-example-sourced.render-program.dot) | the page example's program view, raw |
| [`runs/not1.render-program.dot`](runs/not1.render-program.dot), [`runs/not2.render-program.dot`](runs/not2.render-program.dot), [`runs/not1-vs-not2.cmp.txt`](runs/not1-vs-not2.cmp.txt) | the negations, and their `cmp` |
| [`runs/corpus-sweep.render-program.facts-in-two-digraphs.tsv`](runs/corpus-sweep.render-program.facts-in-two-digraphs.tsv) | per corpus program: facts drawn in two non-rule digraphs |
| [`runs/corpus-sweep.render-program.atoms-in-two-digraphs.tsv`](runs/corpus-sweep.render-program.atoms-in-two-digraphs.tsv) | per corpus program: atoms drawn in two fact digraphs |
| [`runs/corpus-sweep.render-program.refused.tsv`](runs/corpus-sweep.render-program.refused.tsv) | the 225 the view refuses |
| [`renders/claims.render-program.1-ontology.dot`](renders/claims.render-program.1-ontology.dot) / [`.svg`](renders/claims.render-program.1-ontology.svg) | the witness's digraph `ontology`, cut and drawn |
| [`renders/claims.render-program.2-facts.dot`](renders/claims.render-program.2-facts.dot) / [`.svg`](renders/claims.render-program.2-facts.svg) | its digraph `facts` |
| [`renders/claims.render-program.3-solve.dot`](renders/claims.render-program.3-solve.dot) / [`.svg`](renders/claims.render-program.3-solve.svg) | its digraph `solve` |
| [`renders/claims.render-program-levi.1-ontology.dot`](renders/claims.render-program-levi.1-ontology.dot) / [`.svg`](renders/claims.render-program-levi.1-ontology.svg) | `--levi`: `ontology`, the fact as octagon `h_1_owns` |
| [`renders/claims.render-program-levi.2-facts.dot`](renders/claims.render-program-levi.2-facts.dot) / [`.svg`](renders/claims.render-program-levi.2-facts.svg) | `--levi`: `facts`, the fact as octagon `(owns Bob Dog)` |
| [`renders/claims.render-program-levi.3-solve.dot`](renders/claims.render-program-levi.3-solve.dot) / [`.svg`](renders/claims.render-program-levi.3-solve.svg) | `--levi`: `solve` |
| [`renders/page-example-sourced.render-program.1-ontology.dot`](renders/page-example-sourced.render-program.1-ontology.dot) / [`.svg`](renders/page-example-sourced.render-program.1-ontology.svg) | the page example's `ontology` |
| [`renders/page-example-sourced.render-program.2-facts.dot`](renders/page-example-sourced.render-program.2-facts.dot) / [`.svg`](renders/page-example-sourced.render-program.2-facts.svg) | its `facts` |
| [`renders/page-example-sourced.render-program.3-solve.dot`](renders/page-example-sourced.render-program.3-solve.dot) / [`.svg`](renders/page-example-sourced.render-program.3-solve.svg) | its `solve` |
| [`renders/claims.kb.dot`](renders/claims.kb.dot) / [`.svg`](renders/claims.kb.svg) | `ein render kb` of the witness |
| [`renders/page-example-sourced.kb.dot`](renders/page-example-sourced.kb.dot) / [`.svg`](renders/page-example-sourced.kb.svg) | `ein render kb` of the page example |
| [`renders/claims.program.one-digraph.MOCK.dot`](renders/claims.program.one-digraph.MOCK.dot) / [`.svg`](renders/claims.program.one-digraph.MOCK.svg) | mock: one digraph |
| [`renders/claims.program.one-digraph-origin.MOCK.dot`](renders/claims.program.one-digraph-origin.MOCK.dot) / [`.svg`](renders/claims.program.one-digraph-origin.MOCK.svg) | mock: one digraph, the group as a colour |
| [`renders/page-example-sourced.program.one-digraph.MOCK.dot`](renders/page-example-sourced.program.one-digraph.MOCK.dot) / [`.svg`](renders/page-example-sourced.program.one-digraph.MOCK.svg) | mock: the page example as one digraph |
| [`renders/not1-and-not2.render-program.1-ontology.dot`](renders/not1-and-not2.render-program.1-ontology.dot) / [`.svg`](renders/not1-and-not2.render-program.1-ontology.svg) | the one drawing both negations get |
| [`renders/claims.side-by-side.png`](renders/claims.side-by-side.png) | ontology, facts, the two mocks and the KB view in a row |

## 6. Re-run

From the repo root, `E=build/review-bin/ein-head-b5fb8d70`, `D=plans/m1f_hypothesis_and_documentation/review/03-drawings/26-L-98`:

```sh
$E render program $D/programs/claims.ein > $D/runs/claims.render-program.dot
$E render program --levi $D/programs/claims.ein > $D/runs/claims.render-program-levi.dot
$E render program $D/programs/page-example-sourced.ein > $D/runs/page-example-sourced.render-program.dot
$E render program $D/programs/not1.ein > $D/runs/not1.render-program.dot
$E render program $D/programs/not2.ein > $D/runs/not2.render-program.dot
cmp $D/runs/not1.render-program.dot $D/runs/not2.render-program.dot
$E render kb $D/programs/claims.ein > $D/renders/claims.kb.dot
# one file per digraph, then draw (a file of several digraphs is not one SVG)
python3 -c "import re,sys; t=open(sys.argv[1]).read(); [open(f'{sys.argv[2]}.{i}-{re.match(r\"digraph (\S+)\",p).group(1)}.dot','w').write(p.rstrip()+'\n') for i,p in enumerate([p for p in re.split(r'(?m)^(?=digraph )',t) if p.strip()],1)]" $D/runs/claims.render-program.dot $D/renders/claims.render-program
for f in $D/renders/*.dot; do case $f in *.kb.dot) fdp -Tsvg $f > ${f%.dot}.svg;; *) dot -Tsvg $f > ${f%.dot}.svg;; esac; done
```

## 7. Found while building

- The row reproduces as written.
- **No corpus program has the row's case** (a fact stated in one group and pointed at from another); the drawing in
  two digraphs the corpus does have is of atoms (34 programs) and of a fact written twice, once in each group (2).
- Under `--levi` the one fact is two octagons with two different node ids (`h_1_owns`, `(owns Bob Dog)`), not one
  node id drawn twice.
- The frame rule already reads across groups (the octagon in `facts` is unframed because `ontology` states it); only
  the drawing is split.
- `ein render program examples/syntax/config.ein` exits 1, *no renderable forms*, on a program the loader accepts:
  out of this row's scope, noted only.

## 8. Your call

- **One digraph**: each object once in the whole view; the group kept, if wanted, as a colour (the KB view's
  `--colour-by origin`), since the digraph names carry it today.
- **As is**: one digraph per group (no default is recorded; until ruled, it stays).
