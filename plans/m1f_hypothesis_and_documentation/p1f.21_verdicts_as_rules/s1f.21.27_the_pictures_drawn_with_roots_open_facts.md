# S1f.21.27 — the nine pictures drawn with root's `open` facts

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-25 on the user's answer to review item 77, *"(a)"*
([the README](README.md#the-answers-to-part-2s-items-and-item-87)); **not started**, on the user's word.
**Writes:** nine diagram sources under `docs/ein/`'s `diagrams/` folders and their pictures; three sentences; nothing
in the engine.

## What it is

[V-85](findings.md#v-85--nine-pictures-of-the-pages-are-no-longer-what-the-kb-render-draws). Since `ein render kb`
draws the `open` facts root holds (S1f.21.22), nine pictures of the pages are no longer what it draws: their
programs' roots hold `open` facts the pictures leave out. The user ruled **(a)**: a drawing of root shows all its
`open` facts, as built, **and the nine pictures are drawn again with them**. Seventeen other pictures are still what
the binary draws: their programs' roots hold no `open` fact.

| page | picture | the program, the command's option | whole or cut | root's `open` facts |
|---|---|---|---|---:|
| [`user/02-guide/01-objects-and-relations.md`](../../../docs/ein/user/02-guide/01-objects-and-relations.md) l. 136 | `01-tiny-program` | the guide's first program | whole: *draws all of it* | 5, the blind enumerator's |
| [`user/02-guide/02-first-rules.md`](../../../docs/ein/user/02-guide/02-first-rules.md) l. 112 | `02-transitive-fired` | `examples/saturation/transitive/taxonomy.ein` | cut to the facts of `is-a` | 3 |
| `user/02-guide/02-first-rules.md` l. 160 | `02-co-located-fired` | `examples/branching/05_mini_zebra.ein` | cut to the `co-located` facts | 12, the rule `guess`'s |
| [`user/02-guide/03-rule-families.md`](../../../docs/ein/user/02-guide/03-rule-families.md) l. 73 | `03-range-elimination` | `examples/zebra2.ein` | cut to five facts | 130, `guess` 125 and `total-owed` 5 |
| [`reasoning/03-rule-evolution/01-rules-as-data.md`](../../../docs/ein/reasoning/03-rule-evolution/01-rules-as-data.md) l. 91 | `01-instance-of-instance` | the page's program, `--fold` | whole: *every fact of it* | 18 |
| [`reasoning/02-kb-evolution/01-firing.md`](../../../docs/ein/reasoning/02-kb-evolution/01-firing.md) l. 107 | `01-before-firing` | the page's program without its rule, `--fold` | cut: without the declarations' facts | 22 |
| `reasoning/02-kb-evolution/01-firing.md` l. 118 | `01-after-firing` | the page's program, `--fold` | cut: without the declarations' facts | 21 |
| [`reasoning/02-kb-evolution/02-saturation.md`](../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md) l. 342 | `02-note-fixpoint` | `examples/ein-bugs/binding-key-both.ein`, `--fold` | cut: without the declarations' facts, the activators and `(src Y)` | 2 |
| [`reasoning/02-kb-evolution/03-absent-and-naf.md`](../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md) l. 104 | `03-soup-saturated` | the page's program, `--fold`; flagged | cut: without the declarations' facts | 61 |

The list, each page's program copied, today's drawing beside the page's, and who wrote each `open` fact, are review
item 77's, in `git log` at `bc5146bb` (`plans/…/review/part-2/77-what-a-drawing-of-root-shows/`). The line numbers
are the item's; a page moved since is read again.

**Three sentences** say a drawing draws what it no longer does: the guide's *draws all of it* (`01-objects-and-relations.md`),
two lines of `ein.rs/08-reasoning-in-code/01-static-ir.md` on *zebra2*'s views, and two of
`user/04-reference/05-cli.md`.

## Tasks

1. **Each picture drawn again** from its program as the page names it, with root's `open` facts: a whole picture is
   what `ein render kb` draws; a cut picture keeps its cut, and what it keeps of root's `open` facts beside the cut
   is the stage's to choose, recorded as a default (with an id the coordinator gives) and said on the page where it
   cuts. Each source is a file in its page's `diagrams/` folder, drawn by `python3 utils/render_diagrams.py`
   (Q-M1f.368, `utils/check_doc_blocks.py`). A source the tool does not write changes through its generator.
2. **The three sentences** say what the drawing draws now.
3. **Each page's words around its picture** read again: a page that says what a picture shows says it of the new one,
   in its perspective's languages (Q-M1f.370), and the user's pages show the `open` facts as Ein.
4. **What checks it**: `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`,
   `utils/check_release_voice.py`, `utils/check_generated.py`; and each picture compared by hand with what
   `ein render kb` draws today, the comparison in the record. No engine code changes, so the link check and these are
   the stage's gate (the gate's cadence for pages).

## Record

*Not started.*
