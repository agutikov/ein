# S1f.21.27 — the nine pictures drawn with root's `open` facts

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-25 on the user's answer to review item 77, *"(a)"*
([the README](README.md#the-answers-to-part-2s-items-and-item-87)); **built 2026-11-25** in a worktree
(`41ef4922`, `f9bb5e54`, `6a3ae275`; [the record](#record)), one default taken (Q-M1f.700), one finding (V-111).
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

**Built** 2026-11-25 in a worktree on master's `9994ac94`, pages only, three commits and this record:

- `41ef4922` — the guide's four pictures drawn again, and their words (`01-objects-and-relations.md`,
  `02-first-rules.md`, `03-rule-families.md`);
- `f9bb5e54` — the reasoning pages' five pictures drawn again, and their words (`01-firing.md`, `02-saturation.md`,
  `03-absent-and-naf.md`, `01-rules-as-data.md`), with the two code pages that say how each is drawn
  (`ein.rs/08-reasoning-in-code/02-saturation-and-negation.md`, `04-rule-evolution.md`);
- `6a3ae275` — the other two of the three sentences (`ein.rs/08-reasoning-in-code/01-static-ir.md`,
  `user/04-reference/05-cli.md`).

No engine code and no test changed; no engine was built. The binary was the main checkout's release build
(`ein 0.1.0`, `EIN_STDLIB` the worktree's `stdlib/`, sha256 `292894be…`).

### How each picture was drawn

Each program as the page names it, copied to a scratch directory (`tiny.ein` from the guide's block, `kinds.ein` from
`04-rule-evolution.md`, the firing and soup programs from `02-saturation-and-negation.md`, the first without its rule;
the four corpus files as they are; each the same as item 77's copy at `bc5146bb`, its header aside), was drawn with
`ein render kb` (the guide's) or `ein render kb --fold` (`reasoning/`'s). A **whole** picture is that output, its blank
and `//` lines dropped as the nine sources always had them. A **cut** picture was made by a scratch tool,
`kbcut.py`, which reads `ein render kb`'s unfolded DOT back into the facts it draws, keeps a chosen set of the held
facts, and draws them again by `ein-render`'s `fact_graph.rs` rules (`body`, `in_contexts`, `layers`, `ctx_frames`),
folded or not: the facts a kept fact points at come with it, held ones outside every frame and the others in the frame
named for what points at them, and the layers are computed again. **Its check**: with nothing left out it gives back
`ein render kb` and `ein render kb --fold` byte for byte on all nine programs (18 comparisons, zebra2's 2 623 lines
among them). The tool is not in the project; it and its runs are in the session's scratch only.

### The nine, and each beside what `ein render kb` draws today

"Lines" are a source's lines that draw a fact, an arrow or a frame (its header and layers aside); "in the drawing" is
how many of them today's `ein render kb` output holds as they are. Root's `open` facts are the binary's count today,
`grep -c '"open" -> '` of the unfolded output, each equal to the table's above.

| picture | whole or cut | what it draws now | lines · in the drawing | by eye |
|---|---|---|---:|---|
| `01-tiny-program` | whole | the five facts the program writes and root's 5 hypotheses (`by open`), their `nation-loc` facts in `open ctx` | 59 · 59, layers the same | PNG byte-identical to today's |
| `02-transitive-fired` | cut to the facts of `is-a`, and the `open` facts at them | 3 held `is-a` facts, 3 `is-a` hypotheses in `open ctx`, their 3 `open` facts | 37 · 37 | the same octagons, arrows and frame as today's, without the declarations and `(transitive is-a)` |
| `02-co-located-fired` | cut to the held `co-located` facts, and the `open` facts at a `co-located` fact | 6 held `co-located` facts (3 now under an `open` fact, outside the frame), 9 hypotheses in `open ctx`, guess's 12 `open` facts | 100 · 100 | as today's, without the `not` facts, their frame and the rest |
| `03-range-elimination` | cut to five facts, and the `open` facts at what it draws | the 4 `not` facts and their facts in `not, open ctx`, `(color-loc Yellow House-1)` (under an `open` fact now, outside the frame), guess's 5 `open` facts at them, `(open color-loc)` by `std.algebra.total-owed` | 54 · 54 | the cut's PNG read; today's whole drawing drawn too (8100×134 px at 30 dpi, a strip too wide to read whole), so this one is held by the line count |
| `01-instance-of-instance` | whole (*every fact of it*) | the whole of `--fold`: the old picture's facts, the four activators `clash-0`/`clash-2` of `is-a` and `is-a*` (V-93's four), the 18 hypotheses in `open ctx`, each with its loop `open by open` | 127 · 127, layers the same (none: the arrows go round) | PNG byte-identical to today's |
| `01-before-firing` | cut without the declarations' facts | the old picture's five arrows and 22 hypotheses, each an octagon in `open ctx` with its loop `open by open`, `(near Norwegian House-2)` among them | 119 · 119 | as today's `--fold`, the four declarations' octagons and loops gone |
| `01-after-firing` | cut without the declarations' facts | six arrows (`near … by neighbour` dashed) and 21 hypotheses, `(near Norwegian House-2)` no longer one | 115 · 115 | likewise |
| `02-note-fixpoint` | cut without the declarations' facts, the activators and `(src Y)` | the old picture's six arrows and loops and the 2 hypotheses `(holds 2)` and `(holds Y)` in `open ctx`, so `holds` and `Y` are boxes now | 18 · 18 | as today's, the `note ctx` frame, `src`, `edge`'s `note` arrows and the declarations gone |
| `03-soup-saturated` | cut without the declarations' facts | the old picture's nine arrows and loops and the 61 hypotheses in `open ctx`, `(allergic Ann)` among them | 314 · 314 | as today's, the declarations gone |

V-93's two differences are gone with the redraw: `02-co-located-fired` now has today's notes,
`(co-located Cat Tea)` *by symmetric* and `(co-located Cat Alice)` *by transitive*, and `01-instance-of-instance` today's
four activators. Every cut keeps every `open` fact of today's drawing that points at something it keeps (the default
below). Only zebra2's leaves some out, 124 of 130, about facts and relations the cut does not draw; `taxonomy.ein`'s
keeps 3 of 3, the mini-zebra's 12 of 12, and the four cuts of `reasoning/` all of theirs.

### The three sentences, as written

- **The guide's** (`user/02-guide/01-objects-and-relations.md`): *Save it as `tiny.ein`, and `ein render kb tiny.ein`
  draws all of it as one graph, every atom once, and beside it the facts Ein may guess*; the paragraph after says what
  the five are, why the program gets them (the blind enumerator, no label read), lists them as Ein from
  `ein saturate tiny.ein --dump`, links the hypotheses page, and *So far nothing is derived* became *So far nothing
  else is derived: every other fact is one we stated*.
- **`ein.rs/08-reasoning-in-code/01-static-ir.md`**: *`ein render constraints examples/zebra2.ein` draws the L2 facts
  of the saturated root, the facts about relations, every fact with a relation of the program under an argument arrow
  (`ein_infer::slice::about_relations`), root's five `(open R)` facts among them, what it owes, one for each relation
  `std.algebra.total-owed` owes; and `ein render kb` the L0 and L1 with them, and root's 125 hypotheses, `(open F)`
  facts the rule `guess` writes, each pointing at an L1 fact. `ein saturate examples/zebra2.ein --dump` lists the 130
  `open` facts.* (5 and 125 re-taken: `"open" ->` in `ein render constraints`' output, `by guess` in `ein render kb`'s.)
- **`user/04-reference/05-cli.md`**, its two rows: `ein render constraints FILE` — *every fact root holds at its
  fixpoint with a relation of the program under an argument arrow, what root owes, `(open R)`, among them, with what it
  points at*; `ein render kb FILE` — *a knowledge base, loaded and saturated, as one digraph, the `open` facts it holds
  at its fixpoint among its facts, what it owes and its hypotheses (Q-M1f.572)*.

### The words around each picture (task 3)

The guide's three cuts each say, where they cut, that the cut keeps the facts of `open` that point at what it draws,
and show those facts as Ein with the command that lists them (`ein saturate … --dump`): the three of `taxonomy.ein`,
the twelve of the mini-zebra (nine in the frame, the three held ones named), the six of zebra2 (`(open color-loc)` read
as what root owes, linked to the obligations page). The reasoning pages say it in words and mathematics, no command,
path or code: the hypotheses are $\operatorname{open}(f)$ written at the fixpoint (linking hypothesis generation § 2
and § 5), and for each picture which facts they are, as a set where it fits (`01-rules-as-data.md`'s
$\lbrace r(x,y) : r \in \lbrace \operatorname{is-a}, \operatorname{is-a}^{*} \rbrace, x \ne y \in \lbrace
\mathsf{Object}, \mathsf{Relation}, \mathsf{Fact}, \mathsf{Rule}\rbrace, r(x,y) \notin K \rbrace$). The firing page
says both pictures are drawn as a fixpoint, that the rule reads none of the hypotheses, and that after the firing they
are one fewer, $\operatorname{near}(\mathsf{Norwegian}, \mathsf{House\text{-}2})$ being held; $\operatorname{fire}(K,
\xi) = K \cup \lbrace \dots \rbrace$ stands, of the facts $K$ holds. The absence page names
$\operatorname{allergic}(\mathsf{Ann})$ as the hypothesis the absence reads and links the static check and § 11.3
(nothing claimed). The rules-as-data page names the contradiction's activators $\operatorname{clash}_0$ and
$\operatorname{clash}_2$ (linking cores), which the picture always drew less than the binary did. The alt text of each
of the nine says what it now shows. The code pages say each picture keeps root's `open` facts, with the counts and the
command that re-takes them (22 and 21, 2, 61, 18).

### Checks

On `6a3ae275`: `python3 utils/render_diagrams.py --check` (818 diagrams, every picture drawn from its source as it
stands), `python3 utils/check_doc_blocks.py` (rules 1 to 5 hold), `python3 utils/doc_audit.py --links --check --tracked`
(0 findings; an anchor broken on purpose in the working tree was found, so it reads the edits), `python3
utils/check_release_voice.py` (ok), `EIN_BIN=…/ein.rs/target/release/ein python3 utils/check_generated.py` (9
generators, every file as written): each exit 0. No cargo test and no gate was run (the coordinator's, after the
merge). The pictures were drawn by `python3 utils/render_diagrams.py` (Graphviz 16.0.0), nine drawn.

**What reads these pages' text in a test**: `ein-cli/tests/guide_programs.rs` loads every fenced `lisp` block of the
guide; the stage added none (the `open` facts are a `text` block, the dump's lines). `guide_transcripts.rs` pins only
marked transcripts; none was touched. `test_cli.rs`' `the_pages_name_the_reports_schema` reads `05-cli.md` for the
report's schema name, which the stage did not touch. No test reads a picture's source.

### The default taken (Q-M1f.700)

**Q-M1f.700 — a picture cut from a drawing of root keeps the `open` facts about what it draws.** A picture a page cuts
from `ein render kb`'s drawing keeps, beside the facts it is cut to, every `open` fact root holds whose arrow `2` is at
what the cut draws: a fact it keeps, held or denied, a relation's box, or a fact of the relation the cut is cut to,
which it draws in its frame `open ctx`; a cut that leaves out a kind of fact (the declarations' facts, the activators)
keeps every `open` fact that points at nothing it leaves out, here all of them. The page says so where it says what it
cuts. A whole picture is what `ein render kb` draws, every `open` fact in it. *Why*: the ruling is that a drawing of
root shows all its `open` facts; a cut is the page's choice of what to show, and an `open` fact about what it shows is
part of it, while one about nothing it shows is part of what was cut. Applied: zebra2's cut keeps 6 of 130, its
`(open color-loc)` among them since the cut draws the box `color-loc`; the mini-zebra's keeps all 12. **For the
coordinator to file** in `open_questions.md` and the review.

### Findings (V-111)

- **V-111 — nothing holds a page's picture of a knowledge base to what `ein render kb` draws.** A page's picture is
  drawn from its own source, and `render_diagrams.py --check` holds only that the picture is drawn from that source; so
  the nine went stale in silence when the view changed (V-85), and V-93's two drifts before them. A whole picture could
  be held by a check that runs the page's command and compares the source with its output, the blank and comment
  lines aside (here, `01-tiny-program` and `01-instance-of-instance` are exactly that); a cut one by the check this stage
  ran by hand, every line of the source in today's output. Not fixed: a check of the process, outside the stage.

V-112 to V-115 are not used.
