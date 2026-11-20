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
