# Q-M1f.473 — programs a page draws that no file holds

## 1. The question

A drawing a page presents as a command's output is now that command's output on the page's own program
(Q-M1f.467). Two such drawings are of programs **no file of the project holds**, and no page writes out whole: the
hypothesis rule `guess-drink` of the reasoning page on hypothesis generation, and `bag-is-ready` with `forall`'s
expansion written out, of the syntax page on macros. When the pages were drawn again, each program was
**reconstructed** from its page's text, and the drawing made from it. Is that right, or should each program be a file
of the project (so the drawing can be re-made), or the drawing be dropped?

Recorded: [`open_questions.md` § Q-M1f.473](../../../open_questions.md#q-m1f473--programs-a-page-draws-that-no-file-holds),
S1f.23.1's task 2, its reading 7, with [R-20](../../../p1f.23_review_processing/findings.md).

## 2. What is so today

**No file holds either** ([`runs/held-by-a-file.txt`](runs/held-by-a-file.txt): `git grep` of `guess-drink` and
`bag-is-ready` under `examples/`, `tests/`, `stdlib/`, `corpus/` finds nothing). The reconstructions, kept here:

- [`programs/guess-drink.ein`](programs/guess-drink.ein): the declarations of `is-a` and `drinks`, the hrule
  `(hrule guess-drink () :match (and (is-a ?g Guest) (is-a ?d Drink)) :assert (drinks ?g ?d) …)` and three facts for
  it to match. The page says only *"This one says guess, for each guest and each drink, that the guest drinks it"*;
  `ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md` § 1.2 says it is drawn the same way by its name.
- [`programs/bag-is-ready-written-out.ein`](programs/bag-is-ready-written-out.ein): the page's program
  ([`programs/bag-is-ready-with-forall.ein`](programs/bag-is-ready-with-forall.ein), copied from its lisp block) with
  the import dropped and the call replaced by the expansion the page prints, *as the page describes*: *"The program
  with the call replaced by its expansion, and the import dropped, saturates to the same facts."*

**Each reproduces its page's drawing byte for byte**:

| drawing | command | against the page |
|---|---|---|
| `reasoning/02-kb-evolution/diagrams/04-rule-guess-drink.dot` | `ein render rule --fold --name guess-drink` | the same ([`runs/guess-drink.page-vs-binary.diff`](runs/guess-drink.page-vs-binary.diff), exit 0) |
| `user/03-syntax-and-semantics/diagrams/11-rule-bag-is-ready-written-out.dot` | `ein render rule --name bag-is-ready` | the same ([`runs/bag-is-ready-written-out.page-vs-binary.diff`](runs/bag-is-ready-written-out.page-vs-binary.diff), exit 0) |
| `…/11-rule-bag-is-ready-with-forall.dot`, the page's own program | the same | the same ([`runs/bag-is-ready-with-forall.page-vs-binary.diff`](runs/bag-is-ready-with-forall.page-vs-binary.diff), exit 0) |

![guess-drink folded, as the page shows it: is-a arrows from ?g to Guest and ?d to Drink, and the dashed drinks arrow from ?g to ?d it proposes](renders/guess-drink.rule.fold.svg)

![bag-is-ready written out, as the page shows it: an absent cluster around the guard and the body's absence, the body's pattern red](renders/bag-is-ready-written-out.rule.svg)

**And each answers.** `guess-drink` proposes `(drinks Ann Milk)` and `(drinks Ann Tea)`
([`runs/guess-drink.show-hyps.txt`](runs/guess-drink.show-hyps.txt)), R four states
([`runs/guess-drink.solve.txt`](runs/guess-drink.solve.txt)). The two `bag-is-ready` programs answer `(model (ready
Red))` ([`runs/bag-is-ready-with-forall.solve.txt`](runs/bag-is-ready-with-forall.solve.txt),
[`runs/bag-is-ready-written-out.solve.txt`](runs/bag-is-ready-written-out.solve.txt)) and saturate to the same facts
of the program's relations ([`runs/bag-is-ready.facts.diff`](runs/bag-is-ready.facts.diff): empty), as the page says.

## 3. The options

- **(a) reconstructed for the drawing from the page's text** (the default): the drawings stand; the programs live in
  no file, so whoever re-draws them reconstructs them again.
- **(b) a program file added for each**: e.g. under `examples/` with a corpus entry each, the page linking it, so the
  drawing is re-made from a file and the corpus runs it; the two programs here would be those files. Or **the drawing
  dropped**, the page keeping its words. No code changes either way.

## 4. The default

**(a)**, taken by task 2: *"Two programs reconstructed, since no file holds them: `guess-drink`'s (the `is-a` and
`drinks` declarations and the hrule; its `--fold` output equals the old drawing but for two atoms' look) and the
written-out `bag-is-ready` (the page's program with `forall`'s expansion, as the page describes; it loads and answers
`(model (ready Red))`)."*

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing, from the repo root |
| [`programs/guess-drink.ein`](programs/guess-drink.ein) | the reconstruction of `guess-drink`'s program |
| [`programs/bag-is-ready-with-forall.ein`](programs/bag-is-ready-with-forall.ein) | the page's program, copied |
| [`programs/bag-is-ready-written-out.ein`](programs/bag-is-ready-written-out.ein) | the reconstruction of the written-out one |
| [`renders/guess-drink.rule.fold.dot`](renders/guess-drink.rule.fold.dot) / [`.svg`](renders/guess-drink.rule.fold.svg), [`renders/guess-drink.rule.dot`](renders/guess-drink.rule.dot) / [`.svg`](renders/guess-drink.rule.svg), [`renders/guess-drink.rule.on-the-page.dot`](renders/guess-drink.rule.on-the-page.dot) / [`.svg`](renders/guess-drink.rule.on-the-page.svg) | the hrule folded, canonical, and the page's copy |
| [`renders/bag-is-ready-with-forall.rule.dot`](renders/bag-is-ready-with-forall.rule.dot) / [`.svg`](renders/bag-is-ready-with-forall.rule.svg), [`renders/bag-is-ready-with-forall.rule.on-the-page.dot`](renders/bag-is-ready-with-forall.rule.on-the-page.dot) / [`.svg`](renders/bag-is-ready-with-forall.rule.on-the-page.svg) | the rule with the call, and the page's copy |
| [`renders/bag-is-ready-written-out.rule.dot`](renders/bag-is-ready-written-out.rule.dot) / [`.svg`](renders/bag-is-ready-written-out.rule.svg), [`renders/bag-is-ready-written-out.rule.on-the-page.dot`](renders/bag-is-ready-written-out.rule.on-the-page.dot) / [`.svg`](renders/bag-is-ready-written-out.rule.on-the-page.svg) | the rule written out, and the page's copy |
| [`runs/guess-drink.page-vs-binary.diff`](runs/guess-drink.page-vs-binary.diff), [`runs/bag-is-ready-with-forall.page-vs-binary.diff`](runs/bag-is-ready-with-forall.page-vs-binary.diff), [`runs/bag-is-ready-written-out.page-vs-binary.diff`](runs/bag-is-ready-written-out.page-vs-binary.diff) | each page drawing against the binary: empty |
| [`runs/guess-drink.solve.txt`](runs/guess-drink.solve.txt), [`runs/guess-drink.show-hyps.txt`](runs/guess-drink.show-hyps.txt) | `ein solve`, `ein show hyps` |
| [`runs/bag-is-ready-with-forall.solve.txt`](runs/bag-is-ready-with-forall.solve.txt), [`runs/bag-is-ready-written-out.solve.txt`](runs/bag-is-ready-written-out.solve.txt) | `ein solve` of each |
| [`runs/bag-is-ready-with-forall.saturate-dump.txt`](runs/bag-is-ready-with-forall.saturate-dump.txt), [`runs/bag-is-ready-written-out.saturate-dump.txt`](runs/bag-is-ready-written-out.saturate-dump.txt), [`runs/bag-is-ready-with-forall.facts.txt`](runs/bag-is-ready-with-forall.facts.txt), [`runs/bag-is-ready-written-out.facts.txt`](runs/bag-is-ready-written-out.facts.txt), [`runs/bag-is-ready.facts.diff`](runs/bag-is-ready.facts.diff) | `ein saturate --dump` of each, their facts, and the diff: empty |
| [`runs/held-by-a-file.txt`](runs/held-by-a-file.txt) | the `git grep` that finds no file |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt), [`runs/draw-times.tsv`](runs/draw-times.tsv) | the exit codes; Graphviz's layout and seconds |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/53-Q-M1f.473/rerun.sh`.

## 7. Found while building

- Both reconstructions reproduce their drawings byte for byte with today's binary, and the written-out program
  bears out the page's sentence (the same facts).
- A reconstruction is not unique: any program with that rule draws the same rule drawing; `guess-drink`'s three facts
  are the reconstruction's choice, and nothing on the page fixes them.
- `guess-drink`'s program, with no `(solve …)` and no `:no-search`, is answered by its hrule (4 states); the page draws
  only the rule.

## 8. Your call

- **(a)** reconstructed from the page's text, no file (default).
- **(b)** a program file for each (and a corpus entry), or the drawing dropped.
