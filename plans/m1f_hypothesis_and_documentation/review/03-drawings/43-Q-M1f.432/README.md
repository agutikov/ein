# Q-M1f.432 — the KB view leaves nothing out

## 1. The question

The user ruled that `ein render kb` *"renders whole KB"* (Q-M1f.420). Before the stage the view left two kinds of
fact out: a **stored negation**, `(not (owns Eve Dog))`, and a **fact that applies a rule**, `(symmetric friends)`,
whose relation is the name of a rule. The build read *whole* literally: **every** fact the knowledge base holds is
drawn, a stored negation as a fact of `not` whose arrow `2` is at the fact it denies (that fact in the frame
`not ctx` where it is not held), a fact that applies a rule as an octagon whose arrow `1` is at the rule's name. The
program view's `kb` digraph, drawn by the same code, does the same. The other reading, (b), leaves both out again,
in the KB view and the program view's `kb`.

Recorded: [`open_questions.md` § Q-M1f.432](../../../open_questions.md#q-m1f432--what-the-kb-view-leaves-out); the
ruling it reads, [§ Q-M1f.420](../../../open_questions.md#q-m1f420--what-ein-render-program-draws).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; *before* is `build/review-bin/ein-head-b5fb8d70`, the binary
before the stage (the standard library is the same in both). Every KB view suggests `fdp`, and is drawn with it.

Three programs:

- [`programs/small.ein`](programs/small.ein), the drawings' shared small program: `(not (owns Eve Dog))`,
  `(symmetric friends)` applying the rule `symmetric`, and the rule `pet-owner` concluding `(pet-owner ?p)`;
- [`programs/named_like_its_relation.ein`](programs/named_like_its_relation.ein), written for this item: a rule
  `confused` that concludes `(confused ?p)`, so `(confused Ann)` is held and its relation is a rule's name, the case
  topic 04's item 09 met; its claim holds ([`runs/named_like_its_relation.test.txt`](runs/named_like_its_relation.test.txt));
- [`programs/zebra2.ein`](programs/zebra2.ein), a copy of `examples/zebra2.ein`.

What root holds, for each, is `ein saturate --dump`: [`runs/small.saturate-dump.txt`](runs/small.saturate-dump.txt),
[`runs/named_like_its_relation.saturate-dump.txt`](runs/named_like_its_relation.saturate-dump.txt),
[`runs/zebra2.saturate-dump.txt`](runs/zebra2.saturate-dump.txt). What each drawing holds, by the relation under
each fact's arrow `1`, and the sources written beside the facts: `runs/NAME.census.txt`.

### 2.1 The small program: every fact

![ein render kb of small.ein, fdp: every fact an octagon; the fact of not, red, with arrow 1 at the box not and arrow 2 into (owns Eve Dog) in the frame not ctx; the fact of symmetric with arrow 1 at the box symmetric and arrow 2 at friends; the two derived facts of pet-owner, dashed, by pet-owner](renders/small.kb.svg)

([dot](renders/small.kb.dot), 16 octagons, [census](runs/small.kb.census.txt).) The stored negation is the red
octagon at the box `not`, its arrow `2` into `(owns Eve Dog)`, which root does not hold, in the frame `not ctx`; the
fact `(symmetric friends)` is the grey octagon at the box `symmetric`; `(pet-owner Bob)` and `(pet-owner Carol)`,
derived, are dashed, `by pet-owner` beside them. Folded, the same knowledge base:
[`renders/small.kb.fold.svg`](renders/small.kb.fold.svg) (the negation and `(symmetric friends)` stay octagons:
something points at `(owns Eve Dog)`, and `(symmetric friends)` has two arrows).

### 2.2 A rule named like its relation

![ein render kb of named_like_its_relation.ein: the declarations, (contradicts Ann) red, and (confused Ann) dashed with by confused beside it, its arrow 1 at the box confused](renders/named_like_its_relation.kb.svg)

([dot](renders/named_like_its_relation.kb.dot).) `(confused Ann)` is drawn, dashed, `by confused`. **Before**, the
view took it for a fact that applies the rule `confused`, by its name, and left it out:

![before: ein-head-b5fb8d70's ein render kb of the same program: the declarations as loops labelled relation, (contradicts Ann) a loop on Ann; no (confused Ann)](renders/named_like_its_relation.kb.before-b5fb8d70.svg)

([dot](renders/named_like_its_relation.kb.before-b5fb8d70.dot).) The test the view applied was the relation's name:
*a head that names a `Rule`* or `not` (`ein-head-b5fb8d70`'s `kb_dot.rs`, `suppress`, read, not run). A rule with no
parameters that concludes a relation of its own name is one that test cannot tell from an application: on
`small.ein` the before view left out `(pet-owner Bob)` and `(pet-owner Carol)` the same way
([`renders/small.kb.before-b5fb8d70.dot`](renders/small.kb.before-b5fb8d70.dot)).

### 2.3 zebra2: the given conditions are facts that apply a rule

zebra2 writes eight of its fifteen conditions as facts that apply its rule `co-located`,
`(co-located nation-loc Englishman color-loc Red :source "condition (2)")`. Today's view draws every one; the view
before drew seven sources, `(1)`, `(6)`, `(9)` to `(12)` and `(15)`:

<!-- output -->
```text
today   (1) (1) (1) (1) (2) (3) (4) (5) (6) (7) (8) (9) (10) (11) (12) (13) (14) (15)
mock    (1) (1) (1) (1) (6) (9) (10) (11) (12) (15)
```

([`runs/zebra2.kb.census.txt`](runs/zebra2.kb.census.txt),
[`runs/zebra2.kb.left-out.MOCK.census.txt`](runs/zebra2.kb.left-out.MOCK.census.txt); the before view's DOT,
[`renders/zebra2.kb.before-b5fb8d70.dot`](renders/zebra2.kb.before-b5fb8d70.dot), writes the same seven.) Of the 452
facts drawn today, 62 are stored negations and 202 apply a rule (`co-located`, `co-located-negative`, the
`adjacent-via-…` and `disjunctive-prune-…` rules, `std.algebra`'s and `std.bijection`'s): 264 that (b) leaves out.

![ein render kb of zebra2, fdp: 452 octagons and 1 446 arrows, a knot of overlapping numbers and by-labels in which no fact can be followed; the stored negations meet at the box not at the bottom, the facts of is-a* at the top](renders/zebra2.kb.svg)

([dot](renders/zebra2.kb.dot); 13.3 s of `fdp`, [`runs/draw-times.tsv`](runs/draw-times.tsv).) At this size the
whole view is not readable in either form; folded, 250 octagons and 1 042 arrows:
[`renders/zebra2.kb.fold.svg`](renders/zebra2.kb.fold.svg).

## 3. The options

### (a) Nothing left out (the default)

§ 2. The program view's `kb` of a program with no rule, import or macro is then the KB view's drawing but for the
loader's `(relation r)` (item 42), and a fact is drawn whatever its relation is named.

### (b) Stored negations and facts that apply a rule left out

**Mocks**, made by [`tools/mock_leave_out.py`](tools/mock_leave_out.py) from today's DOT (its docstring says what
it edits): every fact whose arrow `1` is at `not` or at a rule's name taken out with its arrows, then every framed
fact nothing left points at; that is the test the view applied before. The names given:

- `small.ein`: `not`, `symmetric`, `pet-owner` → [`renders/small.kb.left-out.MOCK.dot`](renders/small.kb.left-out.MOCK.dot),
  5 facts out, 11 octagons left;
- `named_like_its_relation.ein`: `not`, `confused` → [`renders/named_like_its_relation.kb.left-out.MOCK.dot`](renders/named_like_its_relation.kb.left-out.MOCK.dot) / [`.svg`](renders/named_like_its_relation.kb.left-out.MOCK.svg);
- `zebra2.ein`: `not` and the 27 rules' names its knowledge base names under a first arrow (listed in the mock's
  second line) → [`renders/zebra2.kb.left-out.MOCK.dot`](renders/zebra2.kb.left-out.MOCK.dot) / [`.svg`](renders/zebra2.kb.left-out.MOCK.svg),
  264 out, 188 octagons and 569 arrows left; folded,
  [`renders/zebra2.kb.fold.left-out.MOCK.svg`](renders/zebra2.kb.fold.left-out.MOCK.svg).

![Mock of option (b) on small.ein: no fact of not, no (owns Eve Dog), no fact of symmetric, no fact of pet-owner; the rest as today](renders/small.kb.left-out.MOCK.svg)

What it costs: the derived `(pet-owner Bob)`, `(confused Ann)` and zebra2's eight given conditions go with the
applications, since a name cannot tell a rule's own relation from an application of it; to tell them apart the view
would have to ask whether a rule with parameters reads the fact, which is the theory's business the kernel does not
read (Q-M1f.436).

## 4. The default

**(a), nothing left out** (the ledger): *every fact the knowledge base holds is drawn … so the program view's `kb`
and the KB view of a program with no rule are one drawing.* Built in `kb_dot.rs`; the page `12-rendering.md`
§ The KB has a row for each.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing below, from the repo root |
| [`tools/mock_leave_out.py`](tools/mock_leave_out.py) | the script that wrote option (b)'s mock sources once |
| [`programs/small.ein`](programs/small.ein) | the drawings' shared small program |
| [`programs/named_like_its_relation.ein`](programs/named_like_its_relation.ein) | written for this item |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| [`runs/named_like_its_relation.test.txt`](runs/named_like_its_relation.test.txt) | `ein test`: its claim holds |
| `runs/NAME.saturate-dump.txt` ([small](runs/small.saturate-dump.txt), [named](runs/named_like_its_relation.saturate-dump.txt), [zebra2](runs/zebra2.saturate-dump.txt)) | `ein saturate --dump`: what root holds |
| `renders/NAME.kb.dot` / `.svg` ([small](renders/small.kb.svg), [named](renders/named_like_its_relation.kb.svg), [zebra2](renders/zebra2.kb.svg)) | `ein render kb`, today |
| `renders/NAME.kb.fold.dot` / `.svg` ([small](renders/small.kb.fold.svg), [named](renders/named_like_its_relation.kb.fold.svg), [zebra2](renders/zebra2.kb.fold.svg)) | `ein render kb --fold`, today |
| `renders/NAME.kb.before-b5fb8d70.dot` / `.svg` ([small](renders/small.kb.before-b5fb8d70.svg), [named](renders/named_like_its_relation.kb.before-b5fb8d70.svg), [zebra2](renders/zebra2.kb.before-b5fb8d70.svg)) | `ein render kb` before the stage |
| `renders/NAME.kb.left-out.MOCK.dot` / `.svg` ([small](renders/small.kb.left-out.MOCK.svg), [named](renders/named_like_its_relation.kb.left-out.MOCK.svg), [zebra2](renders/zebra2.kb.left-out.MOCK.svg), [zebra2 folded](renders/zebra2.kb.fold.left-out.MOCK.svg)) | option (b), mocks |
| `runs/NAME.census.txt` ([small](runs/small.kb.census.txt), [small mock](runs/small.kb.left-out.MOCK.census.txt), [zebra2](runs/zebra2.kb.census.txt), [zebra2 mock](runs/zebra2.kb.left-out.MOCK.census.txt), [named](runs/named_like_its_relation.kb.census.txt)) | each canonical drawing's facts by relation, and its sources |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | each drawing's layout and Graphviz's seconds |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | each command's exit code |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/43-Q-M1f.432/rerun.sh
```

It runs `ein test`, `ein saturate --dump` and `ein render kb` (both forms) with `build/review-bin/ein-head-17085b27`,
`ein render kb` with `build/review-bin/ein-head-b5fb8d70`, writes each census, and draws every `.dot` by `fdp`, the
mocks' from their kept sources.

## 7. Found while building

- **Before the stage, zebra2's KB view left out eight of its fifteen given conditions**, (2) to (5), (7), (8), (13),
  (14): each is written as a fact that applies the rule `co-located`, which the view left out by its relation's name.
  (b) would leave them out again.
- **The name test also left out derived facts**: `(confused Ann)` and `small.ein`'s `(pet-owner Bob)` and
  `(pet-owner Carol)`, whose rule has no parameters and the relation's name. Topic 04's item 09 met the first.
- **zebra2's whole KB view is a knot** (452 octagons, 1 446 arrows; under `--fold` 250 and 1 042): `fdp` lays it out
  in 13 s, but no fact can be followed in the picture, the `by <rule>` notes and the arrows' numbers on top of each
  other. Leaving out (b)'s 264 facts would not make it readable (188 octagons, 569 arrows,
  [`renders/zebra2.kb.left-out.MOCK.svg`](renders/zebra2.kb.left-out.MOCK.svg)).
- `--colour-by relation` gives `relation` and `claims` the same colour in `small.ein` (`#bcbd22`): the palette is
  picked by the name's hash, and two names can share a colour.

## 8. Your call

- **(a) Nothing left out** (default): every fact the knowledge base holds, a stored negation a fact of `not`, an
  application of a rule an octagon at the rule's name.
- **(b) Both left out**, in the KB view and the program view's `kb`, by the relation's name, with what it takes along
  (§ 3).
