# 75 · The defaults S1f.21.22 took, and what a drawing of root shows

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.22_what_the_pages_found_in_the_build.md#record)

S1f.21.22 is done: the binary now does what two of your rulings say and it did not. `ein render kb` and
`ein render constraints` draw the `open` facts root holds (Q-M1f.572), and a written `(open …)` that disagrees
with the two lists or a closure is a load error, as a rule's is (Q-M1f.578). These are its seven defaults, and
one question that is yours.

## The question: does a drawing of root show the guesses?

**What is built**: a drawing of root draws every `open` fact root holds, and where nothing else proposes, those
are the blind enumerator's guesses. The four-line program of the guide's first page draws five hypotheses; the
kb view of *zebra2* draws 130 `open` facts. `ein render kb` has no option that leaves them out (default 3).

**What it does to the pages**
([V-85](../../../findings.md#v-85--nine-pictures-of-the-pages-are-no-longer-what-the-kb-render-draws)): nine
pictures no longer match what the binary draws, four of the guide and five of the reasoning pages, and three
sentences say a drawing draws what it no longer does. Seventeen others still match.

- **(a)** As built: a drawing of root shows all its `open` facts. The nine pictures are drawn again with them,
  the guide's first among them. **Default.**
- **(b)** A drawing has the view a listing has: an option on `ein render kb` leaves the `open` facts out, as `-P`
  does for a state's listing (Q-M1f.577), and the nine pages draw their pictures with it and say so.
- **(c)** A drawing of root leaves out the blind enumerator's guesses by default and shows what a rule or an
  obligation wrote, with an option for all of them.

The pages are not touched until you answer: a picture drawn twice is the cost of guessing.

## What else you should know

- **The finding in flight did depend on the count of workers**
  ([V-77](../../../findings.md#v-77--the-hypothesis-found-in-flight-may-depend-on-the-count-of-workers)): a probe
  with eight first nodes named one hypothesis at one job and another in two of six runs at sixteen. The first in
  the nodes' order is recorded now, and a test holds it.
- **A written `(open …)` against a list or a closure is refused** at load, and the line says where it is
  written. No corpus program met it.
- **The claim row**: where a class lists an `open` fact the row says *not printed: these classes list an open
  fact, which no claim lists* (default 1).
- **The shape golden**: 434 lines moved, all of views drawn after saturation, and the seven of the example whose
  `outside` row was wrong.

## The seven

| # | question | what was asked | the default taken | the other reading |
|---|---|---|---|---|
| 1 | [Q-M1f.666](../../../../open_questions.md#q-m1f666--what-the-claim-row-prints-where-a-class-lists-an-open-fact) | what `as :expect` prints where a class lists an `open` fact, or the absence of one | *not printed: these classes list an open fact, which no claim lists*, the first of the reasons a claim is not printed; an `open` under an ordinary relation is a name, and the claim is printed | no row at all; or a claim grammar that takes `open`, which the standing rule that no claim lists one refuses |
| 2 | [Q-M1f.667](../../../../open_questions.md#q-m1f667--which-of-roots-open-facts-the-constraints-view-keeps) | which of root's `open` facts `ein render constraints` keeps | `(open R)`, a fact about a relation by the slice's own definition (Q-M1f.523), and no `(open F)`, whose argument is a fact | every `open` fact root holds, outside the slice's definition |
| 3 | [Q-M1f.668](../../../../open_questions.md#q-m1f668--a-drawing-without-its-open-facts) | a view of a drawing without its `open` facts | none: `ein render kb` has no option that leaves them out; `-P` is a listing's (Q-M1f.577) | a `-P`-like option on `ein render kb`, positive facts alone |
| 4 | [Q-M1f.669](../../../../open_questions.md#q-m1f669--the-shape-goldens-view-of-roots-fixpoint-against-the-loaded-root) | the shape golden's `kb-since` view, root's fixpoint against the loaded root | drawn with root's `open` facts, as the commands draw root, so its lines move with the two renders' | a bare saturation, so only the constraint views' lines move |
| 5 | [Q-M1f.670](../../../../open_questions.md#q-m1f670--where-the-load-error-places-a-written-open-fact) | where the load error places a written `open` fact | *written at FILE:L:C*, the position of its head's word, a top-level fact keeping none of its own (07-diagnostics.md § 1.5), as a written closure is placed; a closure conflict at the closure where it is written, else at the written fact | the list's position alone, with no word on where the fact is |
| 6 | [Q-M1f.671](../../../../open_questions.md#q-m1f671--when-a-workers-finding-in-flight-is-said-on-stderr) | when a worker's in-flight finding is said on stderr | when the fan-out reads its node back, in order, with the hypothesis recorded; a node computed on the caller says it at once, as before | said by the worker at once, the hypothesis alone recorded in order |
| 7 | [Q-M1f.672](../../../../open_questions.md#q-m1f672--the-rendering-pages-layers-picture) | `12-rendering.md`'s layers picture, whose program's root holds 56 guesses of the blind enumerator | the program states `(solve :no-search)`, so the picture is still what the binary draws, byte for byte | the picture redrawn with the 56 hypotheses |

## Your call

The letter for the question, and a number with what you want otherwise for a default, in
[`ANSWERS.md`](../ANSWERS.md) under item 75.
