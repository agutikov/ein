# L-107 — the extent's drawing: a fact no frame can mark

## 1. The question

The knowledge-base page draws the **extent** of the relation `left-of`: every fact whose first arrow points at the
atom `left-of`, held or not. Two of the three facts drawn, $f_3$ and $f_4$, are held; the third, $g$, is held by no
knowledge base and nothing points at it — it is drawn only to show that the extent is more than what is held.
Before the drawing conventions were ruled, $g$ was told apart by being the one **empty** octagon among filled ones.
The ruled conventions (Q-M1f.395, Q-M1f.398) are: every object drawn once; a dashed frame goes around the objects
that are **in a context only** (pointed at and not held); a held object is in no frame; **no octagon is filled**.
So the filling is gone, and nothing in those conventions can mark $g$: it is not in a context (nothing points at
it), so no frame; and no filling. The drawing as built marks it **grey**, a mark the rules do not give, and keeps
the old **unlabelled dashed frame around the extent** — which now reads against the convention, since a dashed
frame says *in a context and not held* and $f_3$, $f_4$ inside it are held. Which drawing should the page have?

Recorded: [`../../findings.md`](../../../findings.md#the-reasoning-pages-reading-order-and-two-sense-words), row
**L-107** (P1f.22's N-50); its default under
[§ Defaults nested facts took](../../../findings.md#defaults-nested-facts-took).

The drawing is **hand-written** (no generator, no `ein` command draws it), so every option is an edit of its DOT.
Nothing here is an engine run: every file is a DOT and its picture drawn with `dot -Tsvg` (Graphviz 16.0.0 here).

## 2. What is so today

The source is `docs/ein/reasoning/01-static-ir/diagrams/01-relation-extent.dot`, copied as
[`renders/01-as-built.dot`](renders/01-as-built.dot); its picture is the page's own
[`renders/01-as-built.svg`](renders/01-as-built.svg) (copied, with its stamp; drawing the copy again gives the same
picture). It is shown on
[`01-knowledge-base.md` § The relation, its extent, a fact of it](../../../../../docs/ein/reasoning/01-static-ir/01-knowledge-base.md#the-relation-its-extent-a-fact-of-it).

![as built: g grey, the dashed frame around f3, f4 and g](renders/01-as-built.svg)

- $g$'s octagon, its first arrow, its two argument arrows and its letter are `#888888` grey; $f_3$, $f_4$ black.
- `subgraph cluster_extent { style="rounded,dashed"; color="#888888"; … }` frames $f_3$, $f_4$, $g$, with no label.
- No octagon is filled.

The page's text beside it, today (quoted): its image line,

```markdown
![The relation left-of, three facts of its extent of which two are held and one, grey, is not, and two facts about it](diagrams/01-relation-extent.svg)
```

and the words under it:

> The first arrows of $f_3$, $f_4$ and $g$ meet at the atom $\mathsf{left\text{-}of}$, and the dashed frame around
> the three is the part of the extent drawn: every other fact of $\operatorname{left-of}$ is in it too. $f_3$ and
> $f_4$ are held, and are the held extent; $g$, drawn grey, is a fact of the relation that the knowledge base does
> not hold, and that no held fact points at. $p$ and the declaration $d_1$ point at the same atom with their second
> arrows: each is one fact about the relation, and neither is in its extent.

And the page's drawing convention, at its head (§ The graph), quoted:

> An octagon that is only pointed at, and not held, stands inside a dashed frame, named for the relation of the
> facts that point at it; a held one stands in no frame.

So on one page a dashed frame means *in a context only, named* (every other figure) and *the part of the extent
drawn, unnamed, around held facts* (this one).

### Before the filling was dropped

`git show 550317f8:docs/ein/reasoning/01-static-ir/diagrams/01-relation-extent.dot` →
[`renders/00-before-filling-dropped.dot`](renders/00-before-filling-dropped.dot),
[`renders/00-before-filling-dropped.svg`](renders/00-before-filling-dropped.svg); its diff to today,
[`runs/diff-as-built-vs-00-before-filling-dropped.diff`](runs/diff-as-built-vs-00-before-filling-dropped.diff)
(the `style=filled, fillcolor="#d9d9d9"` went from the octagons, and `g`'s `style=solid` became the grey).

![before: f3, f4, p, d1 filled grey, g empty, the same dashed frame](renders/00-before-filling-dropped.svg)

The page's text then (at `550317f8`), quoted:

> The first arrows of $f_3$, $f_4$ and $g$ meet at the atom $\mathsf{left\text{-}of}$, and the dashed frame around
> the three is the part of the extent drawn: every other fact of $\operatorname{left-of}$ is in it too. $f_3$ and
> $f_4$, filled, are held, and are the held extent; $g$, empty, is a fact of the relation that the knowledge base
> does not hold.

and its convention: *"A filled octagon is held, an empty one is only pointed at. An octagon that an argument arrow
of a fact points at stands inside a dashed frame, named for the relation of the facts that point at it."*

### The first trial of the frame (for option 2)

When the frame was first tried (the frame then was **around the held facts**, a frame $K$), the extent could not
keep its own frame, and the trial drew the extent as **its three first arrows, in blue**:
`git show b487d5ba:plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/diagrams/frame-relation-extent.dot`
→ [`renders/00b-first-trial-of-the-frame.dot`](renders/00b-first-trial-of-the-frame.dot),
[`.svg`](renders/00b-first-trial-of-the-frame.svg). That frame $K$ was later re-ruled away; only the blue first
arrows are what option 2 takes from it. Its note then (from the deleted `contexts.md` § 14, read, not run):
*"The extent's figure had a dashed frame around the extent, of which two facts are held and one is not; that set
and K overlap, and such frames cannot both be drawn. In the trial the extent is its three first arrows, in blue,
which is what defines it."*

## 3. The options

All six pictures side by side, labelled: [`renders/all-six-montage.png`](renders/all-six-montage.png) (a PNG made
from the same DOTs, for comparing at a glance). Each mock keeps the as-built layout: where the frame is dropped,
the cluster is kept with `style=invis` so the nodes stay where they were; that line is a layout aid, not a frame.

### Option 1 — as built: $g$ grey, the frame kept (the default)

[`renders/01-as-built.svg`](renders/01-as-built.svg). Changes nothing. Costs: a grey mark the rules do not give, and
an unlabelled dashed frame around two held facts, against the page's own convention a few paragraphs up. The page
text stays as quoted in § 2.

### Option 2 — the extent as its first arrows, no frame

[`renders/02-first-arrows-no-frame.MOCK.dot`](renders/02-first-arrows-no-frame.MOCK.dot) /
[`.svg`](renders/02-first-arrows-no-frame.MOCK.svg). **A mock**: the as-built DOT with the frame made invisible and
the three first arrows (`left-of` → $f_3$, $f_4$, $g$) blue and heavier; $g$ keeps its grey
([diff](runs/diff-as-built-vs-02-first-arrows-no-frame.MOCK.diff)).

![option 2: no frame, the three first arrows blue, g still grey](renders/02-first-arrows-no-frame.MOCK.svg)

The dashed frame then means one thing on the page again. What the extent *is* (the facts whose first arrow is at
the atom) is what is drawn. $g$ is still told apart only by grey.

A variant with $g$ not grey either:
[`renders/02b-first-arrows-no-frame-g-plain.MOCK.dot`](renders/02b-first-arrows-no-frame-g-plain.MOCK.dot) /
[`.svg`](renders/02b-first-arrows-no-frame-g-plain.MOCK.svg)
([diff](runs/diff-as-built-vs-02b-first-arrows-no-frame-g-plain.MOCK.diff)). Every mark then is a ruled one, and the
drawing alone no longer tells $g$ from $f_3$, $f_4$: only the text and the formula $\operatorname{ext}_K = \lbrace
f_3, f_4\rbrace$ say which are held.

![option 2b: no frame, blue first arrows, g drawn like f3 and f4](renders/02b-first-arrows-no-frame-g-plain.MOCK.svg)

The page's sentence would become something like (a mock of the wording, not on any page):

> The first arrows of $f_3$, $f_4$ and $g$, drawn blue, meet at the atom $\mathsf{left\text{-}of}$: read
> backwards, they are the part of the extent drawn, and every other fact of $\operatorname{left-of}$ is in it too.
> $f_3$ and $f_4$ are held, and are the held extent; $g$[, drawn grey,] is a fact of the relation that the
> knowledge base does not hold, and that no held fact points at.

The blue is a colour of its own too, but it marks arrows, not a fact's status, and the KB view already colours a
relation's arrows by relation.

### Option 3 — $g$ left out: the figure shows the held extent only

[`renders/03-g-left-out.MOCK.dot`](renders/03-g-left-out.MOCK.dot) /
[`.svg`](renders/03-g-left-out.MOCK.svg): **a mock**, the as-built DOT with $g$'s node and its three arrows deleted
and nothing else ([diff](runs/diff-as-built-vs-03-g-left-out.MOCK.diff)). The frame is still a dashed frame around
held facts.

![option 3: g gone, the frame around f3 and f4](renders/03-g-left-out.MOCK.svg)

With the frame dropped too (the frame around two held facts reads as *in a context only*):
[`renders/03b-g-left-out-no-frame.MOCK.dot`](renders/03b-g-left-out-no-frame.MOCK.dot) /
[`.svg`](renders/03b-g-left-out-no-frame.MOCK.svg)
([diff](runs/diff-as-built-vs-03b-g-left-out-no-frame.MOCK.diff)).

![option 3b: g gone, no frame](renders/03b-g-left-out-no-frame.MOCK.svg)

Every mark is then a ruled one and every octagon drawn is held. What the figure loses is the point the user's
ruling on the extent made (c38b4048 added $g$: *a relation's extent is every fact whose first arrow points at the
relation's atom, held or not*); the page would say it in words and in the formula only, something like (a mock):

> $f_3$ and $f_4$ are held, and are the held extent; the extent itself has every other fact of
> $\operatorname{left-of}$ too, held or not, and the drawing draws only held ones.

and the formula block would lose its $g = \ldots$ line and keep
$\lbrace f_3, f_4 \rbrace \subseteq \operatorname{ext}(\operatorname{left-of})$.

## 4. The default

**Option 1, as built** (*"The extent's $g$, no fact of any knowledge base, is drawn grey (L-107)"*, in
[§ Defaults nested facts took](../../../findings.md#defaults-nested-facts-took)). The commit that dropped the filling
from the twenty hand-written drawings (`58b91dc3`: *"the extent's fact no knowledge base holds is drawn grey"*) gave
$g$ the grey and left the frame as it was; the clash with the frame convention was filed as this row, not ruled.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`renders/01-as-built.dot`](renders/01-as-built.dot) | the tracked source as it is on master today (copy) |
| [`renders/01-as-built.svg`](renders/01-as-built.svg) | the page's picture (copy of the tracked SVG, stamp included) |
| [`renders/00-before-filling-dropped.dot`](renders/00-before-filling-dropped.dot) | the source at `550317f8`, filled octagons, $g$ empty |
| [`renders/00-before-filling-dropped.svg`](renders/00-before-filling-dropped.svg) | drawn |
| [`renders/00b-first-trial-of-the-frame.dot`](renders/00b-first-trial-of-the-frame.dot) | the first frame trial's figure at `b487d5ba` (frame $K$ around held facts; extent as blue first arrows) |
| [`renders/00b-first-trial-of-the-frame.svg`](renders/00b-first-trial-of-the-frame.svg) | drawn |
| [`renders/02-first-arrows-no-frame.MOCK.dot`](renders/02-first-arrows-no-frame.MOCK.dot) | option 2: no frame, blue first arrows, $g$ grey — mock |
| [`renders/02-first-arrows-no-frame.MOCK.svg`](renders/02-first-arrows-no-frame.MOCK.svg) | drawn |
| [`renders/02b-first-arrows-no-frame-g-plain.MOCK.dot`](renders/02b-first-arrows-no-frame-g-plain.MOCK.dot) | option 2, $g$ not grey either — mock |
| [`renders/02b-first-arrows-no-frame-g-plain.MOCK.svg`](renders/02b-first-arrows-no-frame-g-plain.MOCK.svg) | drawn |
| [`renders/03-g-left-out.MOCK.dot`](renders/03-g-left-out.MOCK.dot) | option 3: $g$ deleted, frame kept — mock |
| [`renders/03-g-left-out.MOCK.svg`](renders/03-g-left-out.MOCK.svg) | drawn |
| [`renders/03b-g-left-out-no-frame.MOCK.dot`](renders/03b-g-left-out-no-frame.MOCK.dot) | option 3, frame dropped too — mock |
| [`renders/03b-g-left-out-no-frame.MOCK.svg`](renders/03b-g-left-out-no-frame.MOCK.svg) | drawn |
| [`renders/all-six-montage.png`](renders/all-six-montage.png) | before, as built, 2, 2b, 3, 3b side by side, labelled |
| [`runs/diff-as-built-vs-00-before-filling-dropped.diff`](runs/diff-as-built-vs-00-before-filling-dropped.diff) | `diff -u` as built → before |
| [`runs/diff-as-built-vs-00b-first-trial-of-the-frame.diff`](runs/diff-as-built-vs-00b-first-trial-of-the-frame.diff) | `diff -u` as built → first trial |
| [`runs/diff-as-built-vs-02-first-arrows-no-frame.MOCK.diff`](runs/diff-as-built-vs-02-first-arrows-no-frame.MOCK.diff) | the edit option 2 is |
| [`runs/diff-as-built-vs-02b-first-arrows-no-frame-g-plain.MOCK.diff`](runs/diff-as-built-vs-02b-first-arrows-no-frame-g-plain.MOCK.diff) | the edit option 2b is |
| [`runs/diff-as-built-vs-03-g-left-out.MOCK.diff`](runs/diff-as-built-vs-03-g-left-out.MOCK.diff) | the edit option 3 is |
| [`runs/diff-as-built-vs-03b-g-left-out-no-frame.MOCK.diff`](runs/diff-as-built-vs-03b-g-left-out-no-frame.MOCK.diff) | the edit option 3b is |

## 6. Re-run

From the repo root (`D=plans/m1f_hypothesis_and_documentation/review/03-drawings/06-L-107`):

```sh
cp docs/ein/reasoning/01-static-ir/diagrams/01-relation-extent.dot $D/renders/01-as-built.dot
git show 550317f8:docs/ein/reasoning/01-static-ir/diagrams/01-relation-extent.dot > $D/renders/00-before-filling-dropped.dot
git show b487d5ba:plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/diagrams/frame-relation-extent.dot > $D/renders/00b-first-trial-of-the-frame.dot
for f in $D/renders/*.dot; do dot -Tsvg "$f" > "${f%.dot}.svg"; done
for f in $D/renders/0[0235]*.dot; do diff -u $D/renders/01-as-built.dot "$f" > $D/runs/diff-as-built-vs-$(basename "${f%.dot}").diff; done
```

(The `.MOCK.dot` files are hand edits; the last command re-takes their diffs.)

## 7. Found while building

- The ledger's option 2 says *"as the first trial of the frame drew it"*. That trial (`b487d5ba`) also had a frame
  $K$ around the **held** facts, which was re-ruled away the same day; only its blue first arrows carry over. $g$
  in the trial was told apart by being outside $K$ and unfilled — neither mark exists now, so option 2 still needs
  either the grey (2) or nothing (2b) for $g$.
- The page's own sentence still names the frame (*"the dashed frame around the three is the part of the extent
  drawn"*), so options 2 and 3b each need that sentence rewritten in the same change as the DOT.

## 8. Your call

- **1 — as built** (default): $g$ grey, the unlabelled dashed frame kept.
- **2 — the extent as its first arrows, no frame**: blue first arrows; $g$ grey (2) or plain (2b).
- **3 — $g$ left out**: the held extent only; with the frame (3) or without (3b).
