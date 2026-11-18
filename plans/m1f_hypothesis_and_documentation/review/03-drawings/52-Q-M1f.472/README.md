# Q-M1f.472 — the names of the drawings that changed form

## 1. The question

A drawing's file name on the pages says what it draws: `01-fact-levi` was the fact as the Levi graph (the labelled
octagon, form 2), `01-fact-compact` the fact as a labelled arrow. With the canonical form the default and the labelled
arrow the fold, those words name forms that are gone. When the pages were drawn again, a drawing named for a form took
the form's new name (`levi` → `canonical`, `compact` → `folded`); a drawing whose point was gone was dropped with its
paragraph rewritten (`01-is-a-convention`); and two drawings of what the program view now draws in one digraph were
merged (`12-query-form` and `12-solve-form` into `12-query-view`). Keep those names, or the old ones?

Recorded: [`open_questions.md` § Q-M1f.472](../../../open_questions.md#q-m1f472--names-of-the-drawings-that-changed-form),
S1f.23.1's task 2, its reading 6.

## 2. What is so today

The files, before task 2 (`1418fba5`) and today ([`runs/renames.tsv`](runs/renames.tsv); the commit's stat,
[`runs/task2-commit-stat.txt`](runs/task2-commit-stat.txt)):

| before | today | shown on | before | today |
|---|---|---|---|---|
| guide `01-fact-levi` | `01-fact-canonical` | `02-guide/01-objects-and-relations.md` | ![a labelled octagon nation-loc with arrows 1 and 2](renders/guide.01-fact-levi.at-1418fba5.svg) | ![an unlabelled octagon, its arrows 1, 2, 3 at nation-loc, Norwegian, House-1](renders/guide.01-fact-canonical.svg) |
| guide `01-fact-compact` | `01-fact-folded` | the same | ![an arrow labelled nation-loc](renders/guide.01-fact-compact.at-1418fba5.svg) | ![an arrow labelled nation-loc, the fold](renders/guide.01-fact-folded.svg) |
| guide `05-jack-levi` | `05-jack-canonical` | `02-guide/05-jack-drinks-coffee.md` | [svg](renders/guide.05-jack-levi.at-1418fba5.svg) | [svg](renders/guide.05-jack-canonical.svg) |
| guide `05-jack-compact` | `05-jack-folded` | the same | [svg](renders/guide.05-jack-compact.at-1418fba5.svg) | [svg](renders/guide.05-jack-folded.svg) |
| syntax `01-fact-levi` | dropped | `03-syntax-and-semantics/01-objects.md` | [svg](renders/syntax.01-fact-levi.at-1418fba5.svg) | the page's `kb` drawing and its fold show the facts |
| syntax `01-is-a-convention` | dropped, its paragraph rewritten | the same | [svg](renders/syntax.01-is-a-convention.at-1418fba5.svg): ovals, boxes and dashed empty-headed `is-a` arrows | `is-a` is drawn as any relation (Q-M1f.421); drawn again it would repeat `01-program-kb` |
| — | `01-program-kb-fold`, added | the same | | [svg](renders/syntax.01-program-kb-fold.svg): `ein render kb --fold`, the page's point |
| syntax `12-query-form`, `12-solve-form` | merged into `12-query-view` | `03-syntax-and-semantics/12-query.md` | [query](renders/syntax.12-query-form.at-1418fba5.svg), [solve](renders/syntax.12-solve-form.at-1418fba5.svg): one note each | [svg](renders/syntax.12-query-view.svg): the program view's digraph `query`, the `(solve …)` note and the query's cluster, its goal a pattern |

**No old name is left** anywhere under `docs/`, `README.md` or `examples/`
([`runs/old-names-left.txt`](runs/old-names-left.txt): `git grep` finds none); the new names are linked where the
table says ([`runs/where-shown.txt`](runs/where-shown.txt)). The word *Levi* stays where it is mathematics: the
reasoning page names the hypergraph's **Levi graph**, and the glossary's § *Hyperedge, Levi graph* (renamed from
*Levi-bipartite*) says the drawing is that graph.

The guide's pair, before and today:

![Before: the fact as a labelled octagon nation-loc, its arrows 1 and 2 at Norwegian and House-1](renders/guide.01-fact-levi.at-1418fba5.svg)

![Today, 01-fact-canonical: an unlabelled octagon, its arrow 1 at the box nation-loc, 2 at Norwegian, 3 at House-1](renders/guide.01-fact-canonical.svg)

## 3. The options

- **(a) the new names** (the default): `canonical` and `folded`, the words the ruling and `--fold` use; the dropped
  and merged drawings as above.
- **(b) the old names kept**: `01-fact-levi.dot` would hold the canonical drawing and `01-fact-compact.dot` the folded
  one, `05-jack-*` the same; `01-is-a-convention` drawn again (a repeat of `01-program-kb`) and the two `12-*-form`
  drawn separately. Nothing in the code reads a page drawing's name; the numbering rule (Q-M1f.367) asks only `NN-`
  and a kebab name. No mock: the drawings would be today's under other names.

## 4. The default

**(a)**, taken by task 2: *"a drawing named for a form that is gone takes the form's new name (`levi` → `canonical`,
`compact` → `folded`); `01-is-a-convention` is dropped with its paragraph rewritten (drawn again it would repeat
`01-program-kb`); the two `12-*-form` drawings are one, `12-query-view`, since the program view writes both forms in
its digraph `query`."*

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | the copies, the greps and the drawings, from the repo root |
| `renders/guide.*.at-1418fba5.dot` / `.svg`: [`01-fact-levi`](renders/guide.01-fact-levi.at-1418fba5.dot), [`01-fact-compact`](renders/guide.01-fact-compact.at-1418fba5.dot), [`05-jack-levi`](renders/guide.05-jack-levi.at-1418fba5.dot), [`05-jack-compact`](renders/guide.05-jack-compact.at-1418fba5.dot) | the guide's four before |
| `renders/guide.*.dot` / `.svg`: [`01-fact-canonical`](renders/guide.01-fact-canonical.dot), [`01-fact-folded`](renders/guide.01-fact-folded.dot), [`05-jack-canonical`](renders/guide.05-jack-canonical.dot), [`05-jack-folded`](renders/guide.05-jack-folded.dot) | the guide's four today |
| `renders/syntax.*.at-1418fba5.dot` / `.svg`: [`01-fact-levi`](renders/syntax.01-fact-levi.at-1418fba5.dot), [`01-is-a-convention`](renders/syntax.01-is-a-convention.at-1418fba5.dot), [`12-query-form`](renders/syntax.12-query-form.at-1418fba5.dot), [`12-solve-form`](renders/syntax.12-solve-form.at-1418fba5.dot) | the syntax pages' four before |
| `renders/syntax.*.dot` / `.svg`: [`01-program-kb-fold`](renders/syntax.01-program-kb-fold.dot), [`12-query-view`](renders/syntax.12-query-view.dot) | the syntax pages' two today |
| [`runs/renames.tsv`](runs/renames.tsv) | before, today |
| [`runs/task2-commit-stat.txt`](runs/task2-commit-stat.txt) | what `1d858bb1` did to these files |
| [`runs/where-shown.txt`](runs/where-shown.txt), [`runs/old-names-left.txt`](runs/old-names-left.txt) | where the new names are linked; the old names left (none) |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/52-Q-M1f.472/rerun.sh`.

## 7. Found while building

- Task 2 made the renames with `git mv`, but each file's content changed past git's similarity bound, so the commit
  shows them as deletions and additions ([`runs/task2-commit-stat.txt`](runs/task2-commit-stat.txt)); `git log
  --follow` does not join the two names.
- The page drawings named after a form are copied with `// suggested layout` dropped, so `05-jack-canonical` is laid
  out by `dot` as a strip (R-18, item [55](../55-R-18/README.md)).

## 8. Your call

- **(a)** `canonical`, `folded`; `01-is-a-convention` dropped; `12-query-view` merged (default).
- **(b)** the old names kept.
