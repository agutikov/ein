# Q-M1f.431 — the program view's three parts

## 1. The question

The user ruled that `ein render program` draws **three parts**: the program's knowledge base, drawn as the KB view
draws one; its rules; its query (Q-M1f.420), and that the knowledge base is not split into an `ontology` and a
`facts` digraph. The ruling leaves the details to the build, which took one reading:

- the digraph **`kb`** holds every fact and declaration the program **states**, in the order the loader reads them,
  declarations first, a fact written twice drawn once, as written first; a written check, `(eq …)` or
  `(not (eq …))`, is no fact and is not drawn; an `(import …)` and a `(config …)` draw nothing; a program that states
  no fact has no `kb`;
- **a digraph per rule and hrule**, as `ein render rule` draws one, and per macro its body drawn as a match,
  `macro_<name>`;
- the digraph **`query`**: each query a cluster, its `:goal` **drawn** as a rule's match is and its other clauses
  (`:goal-text`, `:expect`) written in a note beside it; the `(solve …)` clause a note, a flag written alone
  (`:no-search`).

The other reading, (b), keeps the query's `:goal` as **text** in its note, as the view wrote it before. And one
point stays a question in either: a note writes a claim as it is written, `:expect (model (likes Bob Dog))`, so a
fact's text stands in a note's label.

Recorded: [`open_questions.md` § Q-M1f.431](../../../open_questions.md#q-m1f431--the-program-views-three-parts);
the ruling it reads, [§ Q-M1f.420](../../../open_questions.md#q-m1f420--what-ein-render-program-draws).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; *before* is `build/review-bin/ein-head-b5fb8d70`, the binary
before the stage (the standard library is the same in both). Each `.dot` of several digraphs is drawn one picture a
digraph, `NAME.NN-DIGRAPH.svg`: `kb` with `fdp` (its DOT suggests it), the others with `dot`.

The programs:

- [`programs/three_parts.ein`](programs/three_parts.ein), written for this item: an import and a config, two
  declarations, `(owns Bob Dog :source "clue (1)")` and `(owns Bob Dog)` again, a claim
  `(claims Carol (owns Carol Cat))`, two written checks `(eq Bob Bob)` and `(not (eq Bob Carol))`, a macro
  `claimed` called in an hrule's `:match`, a rule, an hrule, two queries with `:expect`, and
  `(solve :hrules (guess-owns))`. It loads and its two claims hold ([`runs/three_parts.test.txt`](runs/three_parts.test.txt));
- [`programs/small.ein`](programs/small.ein), the drawings' shared small program (two rules, a stored negation, a
  fact applying a rule, `(solve :no-search)`);
- [`programs/facts_only.ein`](programs/facts_only.ein), written for this item: no rule, no import, no macro;
- [`programs/zebra2.ein`](programs/zebra2.ein) and [`programs/add.ein`](programs/add.ein), copies of
  `examples/zebra2.ein` and `examples/arith/add.ein` (the calculator's introduction: imports, `let`, nested
  numerals, two queries; [`runs/add.test.txt`](runs/add.test.txt)).

### 2.1 The three parts, on `three_parts.ein`

[`renders/three_parts.program.dot`](renders/three_parts.program.dot) is five digraphs: `kb`, `macro_claimed`, one a
rule (`owner-likes`, `guess-owns`), `query`.

**`kb`**: the two declarations, `(owns Bob Dog)` once with `(1)` beside it (the first written carries the source),
the claim's octagon with its arrow `3` into `(owns Carol Cat)` in the frame `claims ctx`. No octagon for either
written check, nothing for the import or the config.

![ein render program, digraph kb of three_parts: the two declarations as octagons with arrow 1 at relation, (owns Bob Dog) one octagon with (1) beside it, the fact of claims with its arrow 3 into (owns Carol Cat) in the dashed frame claims ctx; nothing for (eq Bob Bob), (not (eq Bob Carol)), the import or the config](renders/three_parts.program.01-kb.svg)

**A rule, and a macro's call as written**: `guess-owns`' match is the macro's call, drawn as it is written, a
pattern whose arrow `1` is at the box `claimed` and whose arrow `3` enters `(owns ?p ?x)` in the frame
`claimed ctx` ([`renders/three_parts.program.04-rule_guess_owns_lhs_rhs.svg`](renders/three_parts.program.04-rule_guess_owns_lhs_rhs.svg));
the macro's body, `(claims ?p ?F)`, is its own digraph
([`renders/three_parts.program.02-macro_claimed.svg`](renders/three_parts.program.02-macro_claimed.svg)).

**`query`**: each query a cluster labelled `query`, its `:goal` drawn as a pattern (an octagon, arrow `1` at the
relation's box, its arguments' boxes and diamonds) beside a note with its other clauses; the `(solve …)` clause a
note.

![ein render program, digraph query of three_parts: two clusters labelled query, each a drawn goal, an octagon with arrows 1, 2, 3 to the box likes or owns and to diamonds or a box, beside a note with :goal-text and :expect (or (model (likes Bob Dog)) …) written as text; a third note :hrules (guess-owns)](renders/three_parts.program.05-query.svg)

The same under `--fold`: [`renders/three_parts.program.fold.dot`](renders/three_parts.program.fold.dot), its `kb`
[`renders/three_parts.program.fold.01-kb.svg`](renders/three_parts.program.fold.01-kb.svg) (`(owns Bob Dog)` a
labelled arrow `owns (1)`, the claim an arrow `claims` from `Carol` into the framed octagon, the declarations, of
four arrows, octagons).

### 2.2 What the `kb` digraph leaves out, against `ein render kb`

The view's `kb` is what the program **states**; `ein render kb` draws what root **holds**, saturated. On
`facts_only.ein` (no rule, no import, no macro) the two differ by one fact,
[`runs/facts_only.program-kb-vs-render-kb.diff`](runs/facts_only.program-kb-vs-render-kb.diff):

<!-- output -->
```text
+  f_e4b012b25a [shape=octagon, label="", …];
+  f_e4b012b25a -> "relation" [xlabel="1", …];
+  f_e4b012b25a -> "owns" [xlabel="2", …];
```

the fact `(relation owns)` the loader adds beside a declaration with labels, which the KB view draws and the
program view does not. On `small.ein` and `three_parts.ein` the KB view has, beyond that, every fact a rule derives
([`runs/small.program-kb-vs-render-kb.diff`](runs/small.program-kb-vs-render-kb.diff),
[`runs/three_parts.program-kb-vs-render-kb.diff`](runs/three_parts.program-kb-vs-render-kb.diff)); every line of each
diff is a `+`, an addition of the KB view's.

### 2.3 A real program: zebra2 and add

zebra2's view is 17 digraphs: `kb`, 15 rules (the hrule `guess` among them), `query`
([`renders/zebra2.program.dot`](renders/zebra2.program.dot)). Its `kb` is 253 arrows canonical, 163 folded:

![ein render program --fold, digraph kb of zebra2, laid out by fdp: the declarations as octagons with arrow 1 at relation, the facts of is-a as labelled arrows is-a, the given facts as labelled arrows with their source's number, right-of (1), drink-loc (9), nation-loc (10), and the facts of bijective and symmetric as octagons](renders/zebra2.program.fold.01-kb.svg)

The canonical one, [`renders/zebra2.program.01-kb.svg`](renders/zebra2.program.01-kb.svg), is a dense knot: 37
octagons of `is-a` meet at one box `is-a`, and `fdp` lays numbers over boxes (1.4 s,
[`runs/draw-times.tsv`](runs/draw-times.tsv)). Its query is one cluster, its `:goal` an `(and …)` of four patterns
drawn, and the `(solve …)` note lists the five hrule calls
([`renders/zebra2.program.17-query.svg`](renders/zebra2.program.17-query.svg)).

`add.ein`'s view is `kb` and `query`: the requests `(add …)` and the `let`s with their nested numerals, each `(n …)`
an octagon in the frame of what points at it, `add ctx`, `let ctx` or `n ctx` ([`renders/add.program.01-kb.svg`](renders/add.program.01-kb.svg)); the imports draw
nothing. Its query notes write each claim's numerals as text:
`:expect (model (sum-of (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8) …))`
([`renders/add.program.02-query.svg`](renders/add.program.02-query.svg)).

### 2.4 Before the stage

`ein-head-b5fb8d70`'s view of `three_parts.ein` ([`renders/three_parts.program.before-b5fb8d70.dot`](renders/three_parts.program.before-b5fb8d70.dot))
was seven digraphs: the two rules first, then `ontology` (the import as a rectangle `std.algebra`, the macro's
parameters and body as facts `(#params ?p ?F)` and `(claims ?p ?F)` in a frame `macro ctx`), `facts`, a digraph
`query` per query, its `:goal` as text in the note, and `solve`:

<!-- output -->
```text
digraph query {
  "query" [shape=note, label=":goal (likes ?p ?x)\n:goal-text \"{?p} likes {?x}\"\n:expect (or (model (likes Bob Dog)) …)"];
}
```

![before: ein-head-b5fb8d70's digraph ontology of three_parts, atoms as rectangles and boxes, the import std.algebra a rectangle, the macro's body and its parameters as labelled octagons in a frame macro ctx](renders/three_parts.program.before-b5fb8d70.03-ontology.svg)

The others before: [`renders/small.program.before-b5fb8d70.dot`](renders/small.program.before-b5fb8d70.dot),
[`renders/facts_only.program.before-b5fb8d70.dot`](renders/facts_only.program.before-b5fb8d70.dot),
[`renders/zebra2.program.before-b5fb8d70.dot`](renders/zebra2.program.before-b5fb8d70.dot),
[`renders/add.program.before-b5fb8d70.dot`](renders/add.program.before-b5fb8d70.dot), each beside its parts' SVGs.

## 3. The options

### (a) As built (the default)

§ 2: `kb` as stated, a digraph per rule, hrule and macro, `query` with each `:goal` drawn and its clauses in a note.

### (b) The query's `:goal` as text in its note, as before

**A mock**, [`renders/three_parts.program.query.goal-as-text.MOCK.dot`](renders/three_parts.program.query.goal-as-text.MOCK.dot):
today's digraph `query` with each query's drawn goal (its octagon, boxes, diamonds and arrows) taken out and the
`:goal` written at the head of its note, as `ein-head-b5fb8d70` wrote it; nothing else edited.

![Mock of option (b): two clusters labelled query, each holding only a note that starts with :goal (likes ?p ?x) or :goal (owns Carol ?x) as text, then :goal-text and :expect; the note :hrules (guess-owns)](renders/three_parts.program.query.goal-as-text.MOCK.svg)

A goal is then a fact's text in a note, which is what (a) took out; it is the more compact.

### What stays a question: a claim's facts written as text

Under (a) and (b) alike a note writes `:expect (model (likes Bob Dog))` as written, so a fact's text stands in a
note's label, as a state's does in the region view (Q-M1f.430, item 41); for `add.ein` it is a 40-character nested
numeral. **A mock** of the other way, [`renders/three_parts.program.query.claims-as-facts.MOCK.dot`](renders/three_parts.program.query.claims-as-facts.MOCK.dot):
each `(model …)` of an `:expect` a frame `model k`, its facts canonical octagons with their own boxes, `(model)` a
frame with a point; the note keeps the `:goal-text`. The frames reuse the dashed, rounded style of a context's frame,
which here would need a look of its own.

![Mock: each query cluster holds its drawn goal, a note with its :goal-text, and a frame per model of its :expect: model 1 and model 2 of the first query, each holding octagons of likes with arrows to boxes Bob, Dog, Carol, Cat; the second query's model 1 an empty frame with a point and model 2 an octagon of owns](renders/three_parts.program.query.claims-as-facts.MOCK.svg)

## 4. The default

**(a), as built** (the ledger), held by `every_object_once.rs`' `the_program_views_kb_is_the_kb_views_drawing` and
its corpus sweep: `kb` is drawn by the KB view's own code, so a program with no rule, import or macro draws the same
`kb` as `ein render kb` but for the loader's `(relation r)` (§ 2.2); the goal is drawn so that no fact's text stands
where the drawing can draw it.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing below, from the repo root |
| [`programs/three_parts.ein`](programs/three_parts.ein), [`programs/facts_only.ein`](programs/facts_only.ein) | written for this item |
| [`programs/small.ein`](programs/small.ein) | the drawings' shared small program |
| [`programs/zebra2.ein`](programs/zebra2.ein), [`programs/add.ein`](programs/add.ein) | copies of `examples/zebra2.ein`, `examples/arith/add.ein` |
| [`runs/three_parts.test.txt`](runs/three_parts.test.txt), [`runs/add.test.txt`](runs/add.test.txt) | `ein test`: every claim holds |
| `renders/NAME.program.dot` and its parts `renders/NAME.program.NN-DIGRAPH.dot` / `.svg`, NAME `three_parts`, `small`, `facts_only`, `zebra2`, `add` ([three_parts](renders/three_parts.program.dot), [small](renders/small.program.dot), [facts_only](renders/facts_only.program.dot), [zebra2](renders/zebra2.program.dot), [add](renders/add.program.dot)) | `ein render program`, today |
| `renders/NAME.program.fold.dot` and its parts ([three_parts](renders/three_parts.program.fold.dot), [small](renders/small.program.fold.dot), [facts_only](renders/facts_only.program.fold.dot), [zebra2](renders/zebra2.program.fold.dot), [add](renders/add.program.fold.dot)) | `ein render program --fold`, today |
| `renders/NAME.program.before-b5fb8d70.dot` and its parts ([three_parts](renders/three_parts.program.before-b5fb8d70.dot), [small](renders/small.program.before-b5fb8d70.dot), [facts_only](renders/facts_only.program.before-b5fb8d70.dot), [zebra2](renders/zebra2.program.before-b5fb8d70.dot), [add](renders/add.program.before-b5fb8d70.dot)) | the view before the stage |
| `renders/NAME.kb.dot` / `.svg` ([three_parts](renders/three_parts.kb.svg), [small](renders/small.kb.svg), [facts_only](renders/facts_only.kb.svg)) | `ein render kb`, for the comparison |
| `runs/NAME.program-kb-digraph.dot` ([three_parts](runs/three_parts.program-kb-digraph.dot), [small](runs/small.program-kb-digraph.dot), [facts_only](runs/facts_only.program-kb-digraph.dot)) | the program view's digraph `kb`, cut out |
| `runs/NAME.program-kb-vs-render-kb.diff` ([three_parts](runs/three_parts.program-kb-vs-render-kb.diff), [small](runs/small.program-kb-vs-render-kb.diff), [facts_only](runs/facts_only.program-kb-vs-render-kb.diff)) | its `diff -u` against `ein render kb` |
| [`renders/three_parts.program.query.goal-as-text.MOCK.dot`](renders/three_parts.program.query.goal-as-text.MOCK.dot) / [`.svg`](renders/three_parts.program.query.goal-as-text.MOCK.svg) | option (b), a mock |
| [`renders/three_parts.program.query.claims-as-facts.MOCK.dot`](renders/three_parts.program.query.claims-as-facts.MOCK.dot) / [`.svg`](renders/three_parts.program.query.claims-as-facts.MOCK.svg) | the claim's facts drawn, a mock |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | each drawing's layout and Graphviz's seconds |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | each command's exit code (a diff that differs exits 1) |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/42-Q-M1f.431/rerun.sh
```

It runs `ein test` on the two programs that state claims, `ein render program` (both forms) on the five programs
with `build/review-bin/ein-head-17085b27` and once with `build/review-bin/ein-head-b5fb8d70`, `ein render kb` on
three, cuts the `kb` digraph out and diffs it, and draws every `.dot`, the mocks' from their kept sources.

## 7. Found while building

- **`(neq …)` cannot be written at a program's top level**: `(neq Bob Carol)` is refused at the parse
  (*unexpected `neq`*), while `(eq Bob Bob)` and `(not (eq Bob Carol))` load. The ledger's reading and
  `12-rendering.md` § The program as written both name `(neq …)` among the written checks the `kb` leaves out; read,
  that sentence is about a form no program can hold there. The program here writes the two that load.
- **A macro's call is no fact**: `(claimed Carol (owns Carol Cat))` at the top level is refused at load (*a fact is
  not expanded, and no fact is of a macro*), so a macro's call is drawn only where a `:match`, `:assert` or `:goal`
  holds it, as written, its relation's box the macro's name (`claimed`), its frame `claimed ctx`.
- **The query clusters are not told apart**: each is labelled `query`, where the read-out numbers them
  `query 1 of 2`, `query 2 of 2`.
- **zebra2's canonical `kb`, laid out by `fdp`, is unreadable at its size**: numbers sit on boxes, 37 arrows meet at
  `is-a`. The folded one reads.
- The program view of `add.ein` has no rule digraph: the calculator's rules are imported, and imports are not
  resolved, so its view is `kb` and `query` alone.

## 8. Your call

- **(a) As built** (default): `kb` as stated, a digraph per rule, hrule and macro, `query` with each goal drawn and
  its other clauses in a note.
- **(b) The goal as text** in its note, as before.
- And, in either: a claim's facts **as text in the note** (today), or **drawn**, a frame a model.
