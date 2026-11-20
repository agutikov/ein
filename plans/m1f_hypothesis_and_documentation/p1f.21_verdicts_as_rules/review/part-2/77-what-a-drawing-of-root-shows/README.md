# 77 · What a drawing of root shows

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-85--nine-pictures-of-the-pages-are-no-longer-what-the-kb-render-draws) ·
[the stage that built it](../../../s1f.21.22_what_the_pages_found_in_the_build.md#record)

**Nine pictures of the pages wait on this answer.**

## What changed

You ruled the `open` facts a part of a state, which every surface shows (Q-M1f.572), and `-P` the listing
without them (Q-M1f.577). S1f.21.14 found `ein render kb` drawing none of them, and S1f.21.22 built it: a drawing
of root now draws every `open` fact root holds. Where no rule and no obligation proposes, those are **the blind
enumerator's guesses**. `ein render kb` has no option that leaves them out.

## What it looks like

The guide's first program, four lines, [`programs/tiny.ein`](programs/tiny.ein):

```lisp
(relation nation-loc Nationality House :why "the {?3} lives in {?4}")
(is-a Norwegian Nationality)
(is-a Englishman Nationality)
(nation-loc Norwegian House-1 :source "condition (10)")
```

| | the page's picture, as it is | what `ein render kb` draws today |
|---|---|---|
| the guide's first program | `runs/page.01-tiny-program.svg`: the five facts the program writes | `runs/tiny.kb.svg`: those, and the hypotheses root holds, each an octagon under `open` in a frame; `runs/tiny.hyps.txt` lists them |
| *zebra2*, the guide's range elimination | `runs/page.03-range-elimination.svg`: five facts, cut by hand from the drawing | `runs/zebra2.kb.svg`: the whole knowledge base with root's `open` facts; `runs/zebra2.open-arrows.txt` counts them |

`sh tools/render.sh`, from the repository's root, draws them with the binary at `ein.rs/target/release/ein`, whose
version is in `runs/version.txt`. The tool and the runs are on this machine only, as every item's are.
`bash tools/render-all.sh` draws all nine pictures the same way on the phase's final binary, and these two come out
of it byte for byte as above: the table below and [*On the built engine*](#on-the-built-engine).

## The nine pictures

Each is on a page as what `ein render kb` draws, and no longer is. Seventeen other pictures still are: their
programs' roots hold no `open` fact.

| page | picture | root's `open` facts, not drawn |
|---|---|---:|
| [`user/02-guide/01-objects-and-relations.md`](../../../../../../docs/ein/user/02-guide/01-objects-and-relations.md) | `01-tiny-program`; its sentence says the command *draws all of it* | 5 |
| [`user/02-guide/02-first-rules.md`](../../../../../../docs/ein/user/02-guide/02-first-rules.md) | `02-transitive-fired` | 3 |
| [`user/02-guide/02-first-rules.md`](../../../../../../docs/ein/user/02-guide/02-first-rules.md) | `02-co-located-fired` | 12 |
| [`user/02-guide/03-rule-families.md`](../../../../../../docs/ein/user/02-guide/03-rule-families.md) | `03-range-elimination`, of *zebra2* | 130 |
| [`reasoning/03-rule-evolution/01-rules-as-data.md`](../../../../../../docs/ein/reasoning/03-rule-evolution/01-rules-as-data.md) | `01-instance-of-instance` | 18 |
| [`reasoning/02-kb-evolution/01-firing.md`](../../../../../../docs/ein/reasoning/02-kb-evolution/01-firing.md) | `01-before-firing`, `01-after-firing` | 21 |
| [`reasoning/02-kb-evolution/02-saturation.md`](../../../../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md) | `02-note-fixpoint` | 2 |
| [`reasoning/02-kb-evolution/03-absent-and-naf.md`](../../../../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md) | `03-soup-saturated` | 61 |

Three sentences say a drawing draws what it no longer does: the guide's *draws all of it*, two lines of
`ein.rs/08-reasoning-in-code/01-static-ir.md` on *zebra2*'s views, and two of `user/04-reference/05-cli.md`.

### Each picture beside what the binary draws today

Each row: the page's picture as it is, copied with its source (`runs/page.NN-name.svg` and `.dot`); the program
the page draws it from, a corpus file or the program a page of
[`ein.rs/08-reasoning-in-code/`](../../../../../../docs/ein/ein.rs/08-reasoning-in-code/README.md) writes out,
copied under `programs/` with a header naming the page and its lines; whether the page's picture is the whole
drawing or cut from it by hand, as the page says; what `ein render kb` draws today, with the option the page names;
and the `open` facts root holds, as the finding's record counted them and as the binary counts them today, in the
dump and in the drawing alike (`runs/open-counts.txt`; each fact with its writer in `runs/<name>.open-facts.txt`).

| picture, its page and line | the page's picture | the program, the command's option | whole or cut | today's drawing | who wrote root's `open` facts (`runs/composed.txt`), and H's note (`runs/<name>.show-hyps.txt`) | root's `open` facts: the record · today |
|---|---|---|---|---|---|---:|
| `01-tiny-program`, [guide 01](../../../../../../docs/ein/user/02-guide/01-objects-and-relations.md) l. 136 | `runs/page.01-tiny-program.svg` | [`programs/tiny.ein`](programs/tiny.ein) | whole: *draws all of it* | `runs/tiny.kb.svg` | the blind enumerator, `blind`: 5 candidates | 5 · 5 |
| `02-transitive-fired`, [guide 02](../../../../../../docs/ein/user/02-guide/02-first-rules.md) l. 112 | `runs/page.02-transitive-fired.svg` | [`taxonomy.ein`](../../../../../../examples/saturation/transitive/taxonomy.ein) | cut *to the facts of `is-a`* | `runs/taxonomy.kb.svg` | `blind`: 3 | 3 · 3 |
| `02-co-located-fired`, [guide 02](../../../../../../docs/ein/user/02-guide/02-first-rules.md) l. 160 | `runs/page.02-co-located-fired.svg` | [`05_mini_zebra.ein`](../../../../../../examples/branching/05_mini_zebra.ein) | cut to the `co-located` facts | `runs/mini-zebra.kb.svg` | the rule `guess`, `hypothesis-rules`: 9 candidates | 12 · 12 |
| `03-range-elimination`, [guide 03](../../../../../../docs/ein/user/02-guide/03-rule-families.md) l. 73 | `runs/page.03-range-elimination.svg` | [`zebra2.ein`](../../../../../../examples/zebra2.ein) | cut to five facts | `runs/zebra2.kb.svg` | the rule `guess` 125 and the obligation `std.algebra.total-owed` 5, `union`: 56 candidates | 130 · 130 |
| `01-instance-of-instance`, [rules as data](../../../../../../docs/ein/reasoning/03-rule-evolution/01-rules-as-data.md) l. 91 | `runs/page.01-instance-of-instance.svg` | [`programs/kinds.ein`](programs/kinds.ein), `--fold` | whole: *every fact of it* | `runs/kinds.kb.svg` | `blind`: 18 | 18 · 18 |
| `01-before-firing`, [firing](../../../../../../docs/ein/reasoning/02-kb-evolution/01-firing.md) l. 107 | `runs/page.01-before-firing.svg` | [`programs/neighbour-no-rule.ein`](programs/neighbour-no-rule.ein), `--fold` | cut: without the declarations' facts | `runs/neighbour-no-rule.kb.svg` | `blind`: 22 | 21 · **22** |
| `01-after-firing`, [firing](../../../../../../docs/ein/reasoning/02-kb-evolution/01-firing.md) l. 118 | `runs/page.01-after-firing.svg` | [`programs/neighbour.ein`](programs/neighbour.ein), `--fold` | cut: without the declarations' facts | `runs/neighbour.kb.svg` | `blind`: 21 | 21 · 21 |
| `02-note-fixpoint`, [saturation](../../../../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md) l. 342 | `runs/page.02-note-fixpoint.svg` | [`binding-key-both.ein`](../../../../../../examples/ein-bugs/binding-key-both.ein), `--fold` | cut: without the declarations' facts, the activators and `(src Y)` | `runs/binding-key-both.kb.svg` | `blind`: 2 | 2 · 2 |
| `03-soup-saturated`, [absence](../../../../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md) l. 104 | `runs/page.03-soup-saturated.svg` | [`programs/soup.ein`](programs/soup.ein), `--fold`; **flagged** | cut: without the declarations' facts | `runs/soup.kb.svg` | `blind`: 61 | 61 · 61 |

The record gave the two firing pictures one row and one figure, 21, which is the root of the program with the rule.
`01-before-firing` is drawn from the program without it (`02-saturation-and-negation.md` lines 77 and 78 say so),
whose root holds one guess more, `(open (near Norwegian House-2))`, the fact the rule derives in the other:
`runs/neighbour-no-rule.open-facts.txt` against `runs/neighbour.open-facts.txt` (V-92, a finding of this stage, in
its record). Where H is smaller than the count, the other `open` facts point at facts root holds or denies: of the
mini-zebra's 12, three point at facts root holds; of *zebra2*'s 130, five are `(open R)`, what root owes, and of the
125 hypotheses 7 point at facts root holds and 62 at facts it denies, which stand in the frame `not, open ctx`
(`runs/composed.txt`).

`programs/soup.ein` is flagged: it loads, and nothing is claimed of its answer. `ein show hyps` says so on stderr's
first line, *WARNING: programs/soup.ein: THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER
(Q-M1f.564)* (`runs/soup.show-hyps.txt`), and the read-out's first row is `WARNING` (`runs/soup.solve.txt`); the
site is the rule `soup-unless-allergic`, whose `absent` reads `allergic`, which the blind enumerator guesses. The
engine before the phase's second part flagged it too (`runs/before-part-2/soup.show-hyps.txt`).

## The question

- **(a)** As built: a drawing of root shows all its `open` facts. The nine pictures are drawn again with them:
  the guide's first picture gains five hypotheses, and its range elimination is cut out of a drawing that holds
  130 more facts. **Default.**
- **(b)** A drawing has the view a listing has: an option on `ein render kb` leaves the `open` facts out, as `-P`
  does for a state's listing. The default stays as built, and the nine pages draw their pictures with the option
  and say so. **The coordinator's recommendation**: the default keeps your ruling to the letter, and a page
  that teaches what a fact is does not have to explain a guess first.
- **(c)** A drawing of root leaves out the blind enumerator's guesses by default, and draws what a rule or an
  obligation wrote; an option draws them all. The nine pictures stay as they are but the two a rule or an
  obligation writes for.

The pages are not touched until you answer.

## On the built engine

**How it was taken.** The binary is the phase's final one, the release build of `7b72bd84` (`runs/version.txt`).
`tools/render-all.sh` re-takes every run below; after the merge, `bash tools/render-all.sh` runs it with the binary
at `ein.rs/target/release/ein` (this time `EIN_BIN` named the final binary, `EIN_REPO` and `ITEM_SRC` the stage's
worktree). For each of the nine pictures it copies the page's picture and its source, and runs, on the picture's
program and with the option its page names (`--fold` for the five pictures of `reasoning/`): `ein render kb`
(`runs/<name>.kb.dot`, drawn as `.svg`), `ein show hyps` (H, and on stderr which writer built it,
`runs/<name>.show-hyps.txt`), `ein saturate --dump` (`runs/<name>.saturate-dump.txt`) and `ein solve`
(`runs/<name>.solve.txt`); each of these three files ends with its command's exit code, 0 in every one, and no
drawing wrote anything on stderr. No oracle, lattice or region is drawn: the question is what `ein render kb` draws
of root. `tools/render.sh`'s files are kept: the two drawings it made come out of `tools/render-all.sh` byte for
byte the same, and its version file and its dump of `tiny.ein` are kept as `runs/version.render-sh.txt` and
`runs/tiny.saturate-dump.render-sh.txt`. With the engine before the phase's second part (`ddf09492`,
`runs/before-part-2/version.txt`) the tool also draws the programs of the two pictures below that differ from today's
drawing in more than the `open` facts, and lists `soup.ein`'s H.

**The finding's figures beside today's** are the last column of the table above: eight are as the record gave
them, and the ninth, `01-before-firing`, is 22 where the record gave the two firing pictures 21, because its program
is the one without the rule.

**How each page's picture stands to today's drawing** (`runs/page-lines.txt`): of the lines of the picture's source
that draw a fact or an arrow, how many today's drawing holds as they are. A picture cut by hand from the drawing
holds none it does not.

| picture | lines that draw a fact or an arrow | in today's drawing, as they are |
|---|---:|---:|
| `01-tiny-program` | 20 | 20 |
| `02-transitive-fired` | 12 | 12 |
| `02-co-located-fired` | 24 | **22** |
| `03-range-elimination` | 32 | 32 |
| `01-instance-of-instance` | 29 | 29 |
| `01-before-firing` | 5 | 5 |
| `01-after-firing` | 6 | 6 |
| `02-note-fixpoint` | 6 | 6 |
| `03-soup-saturated` | 9 | 9 |

Two pictures differ from today's drawing in more than its `open` facts, and both did before the second part, so
neither is this question's (V-93, a finding of this stage, in its record):

- `02-co-located-fired`: the two lines today's drawing does not hold are the notes of two derived facts. The page
  has `(co-located Cat Alice)` *by symmetric* and `(co-located Cat Tea)` *by transitive*; today's drawing has them
  the other way round, and so does the engine before the second part (`runs/before-part-2/page-lines.txt`). A note
  names the rule of a fact's first derivation, which may vary with the order rules fire in.
- `01-instance-of-instance`, which its page calls *every fact of it*: the drawing has four facts more than the
  picture, beside its 18 `open` facts: the activators of `std.contradiction`'s rules `clash-0` and `clash-2` for
  `is-a` and `is-a*`, each a loop *by std.contradiction.clash-of-0* or *-of-2* (`runs/kinds.composed-b.page-diff.txt`;
  `std.algebra` imports `std.contradiction`). The engine before the second part draws the same four
  (`runs/before-part-2/kinds.page-diff.txt`).

### Options (b) and (c), composed by hand

**These drawings are not drawn by the binary.** `tools/compose.py` reads today's DOT back into the facts it draws
and draws what is left again, by the rules `ein render kb` draws by (`ein-render`'s `fact_graph.rs`): **(b)** leaves
out every `open` fact, **(c)** only those the blind enumerator wrote. The DOT names an `open` fact's writer in its
note, *by X* beside the octagon, or under `--fold` the loop's label *open by X*: X is the rule, the obligation, or
`open` for the blind enumerator (Q-M1f.612). Leaving out an `open` fact takes with it its octagon and its arrows,
the facts root does not hold that only it pointed at (the hypotheses in the frame `open ctx`), the frame where
nothing is left in it, and the box `open` where no arrow reaches it; a frame named by two relations loses `open`
from its name; the layers are computed again. Each composed `.dot`'s first line is the comment *composed by hand
from ein render kb's drawing, not drawn by the binary*, and its second says what it left out. Two checks: drawn
again with nothing left out, each of the nine drawings comes back line for line (`runs/compose-check.txt`); and (b)
composed for `tiny.ein` is the guide's picture `01-tiny-program` line for line, the blank and comment lines aside
(`runs/tiny.composed-b.page-diff.txt`). One thing the DOT does not carry: where a held fact that stood in a context
only because an `open` fact pointed at it goes among the facts nothing points at. The tool puts it after them; it
happens under (b) alone, to three facts of the mini-zebra and seven of *zebra2*.

**The guide's first program and the mini-zebra.** For `tiny.ein` the two options draw the same thing, the page's
picture. The second program is `05_mini_zebra.ein`, the guide's `02-co-located-fired`, where the two differ most: (b)
leaves out all twelve of its `open` facts and (c) none.

| program | root's `open` facts, by writer | (b) leaves out | (c) leaves out | the composed drawings |
|---|---|---|---|---|
| `tiny.ein` | 5, the blind enumerator's | the 5, the 5 hypotheses they point at, the frame and the box `open` | the same: (c) is (b) | `runs/tiny.composed-b.svg`, `runs/tiny.composed-c.svg` |
| `05_mini_zebra.ein` | 12, the rule `guess`'s | the 12, the 9 hypotheses only they point at, the frame `open ctx` and the box `open`; the 3 facts root holds that they point at leave the context | nothing: (c) is today's drawing | `runs/mini-zebra.composed-b.svg`, `runs/mini-zebra.composed-c.svg` |
| `zebra2.ein` | 130: the rule `guess` 125, the obligation `std.algebra.total-owed` 5 | the 130, the 56 hypotheses, the frame `open ctx` and the box `open`; 7 facts root holds leave the context, and the 62 it denies go from the frame `not, open ctx` to `not ctx` | nothing | `runs/zebra2.composed-b.svg`, `runs/zebra2.composed-c.svg` |
| the six others | all the blind enumerator's: 3, 18, 22, 21, 2 and 61 | all of them, the hypotheses they point at and the frame `open ctx` | the same: (c) is (b) | `runs/<name>.composed-b.svg`, `runs/<name>.composed-c.svg` |

`runs/composed.txt` has each composition's counts, one line each.

**Why no program shows the three options apart.** Root never holds the blind enumerator's guesses beside a rule's or
an obligation's `open` facts: the blind enumerator writes only where root holds no `open` fact and activates no
obligation (Q-M1f.570, and the word `blind` of
[`03-hypotheses-and-the-answer.md` § 1.2](../../../../../../docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md)).
So on every program either all of root's `open` facts are the blind enumerator's, and (c) draws what (b) draws, or
none is, and (c) draws what (a) draws. Of the nine pictures, seven are of the first kind; the mini-zebra and
*zebra2*, the two option (c) names, are of the second.
