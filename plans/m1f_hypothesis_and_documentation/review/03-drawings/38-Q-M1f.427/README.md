# Q-M1f.427 — how a rule's pattern is drawn, and a `forall`'s guard (R-15)

## 1. The question

`ein render rule` draws a rule as its match and its conclusion. A **pattern**, `(owns ?p ?x)`, is now drawn as a
fact is: an octagon with no name in it, its arrow `1` at its relation's box, or at the variable's diamond where the
relation is a variable, `(?rel ?a ?b)`, and its next arrows at its arguments. The kernel's own forms keep their own
marks: a guard `(neq …)`/`(eq …)` is a dotted `≠`/`=` link, an `(absent …)` a cluster `absent (∄)`, a `(not P)` a red
pattern of `not` whose arrow `2` is at `P`'s node. **No macro is read by its name**: `std.macro`'s `forall`, which the
view drew as a cluster `forall (∀)`, and `unknown` are patterns as called, `(forall ?y G B)` an octagon whose arrow
`1` is at the box `forall` (your ruling that the engine reads no theory's name, Q-M1f.436, is what took the name
out). The other reading: `forall` kept as a cluster of its own.

**R-15**, found while redrawing the pages: drawn as a pattern, `forall`'s arguments are drawn as the facts they are
written as. In `std.bijection`'s `domain-elimination`, the guard `(and (?isa ?b_other ?B) (neq ?b_other ?b))` becomes
an octagon of `and` and an octagon of `neq`, in frames, where a `neq` of a match is the dotted `≠` link, and where the
language says *`eq`, `neq`, `absent`, `and`, `or` and `false` with an argument are never a nested head*. The macro's
arguments are formulas, not facts, and the view draws them as facts because it draws the form as written.

Recorded: [`open_questions.md` § Q-M1f.427](../../../open_questions.md#q-m1f427--how-a-rules-pattern-is-drawn) (put,
not ruled; default (a)); R-15 in [P1f.23's findings](../../../p1f.23_review_processing/findings.md) (the user's).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; every drawing below exists in four ways, side by side or
overlay, canonical or `--fold` (§ 5). `dot` draws a rule (it suggests no layout).

**One rule a pattern kind**, [`programs/patterns.ein`](programs/patterns.ein):

- `keeps`, a plain pattern ([`renders/patterns.rules.01-rule_keeps_lhs_rhs.svg`](renders/patterns.rules.01-rule_keeps_lhs_rhs.svg));
- `symmetric`, the relation a variable: arrow `1` at the diamond `?rel`
  ([`renders/patterns.rules.02-rule_symmetric_lhs_rhs.svg`](renders/patterns.rules.02-rule_symmetric_lhs_rhs.svg));
- `owns-unloved`, a guard and a stored negation in the match: the dotted `≠` between `?x` and `?y`, and `(not (likes
  ?p ?x))` a red octagon whose arrow `1` is at `not` and `2` at `(likes ?p ?x)`, in the frame `not ctx`:

  ![owns-unloved: two patterns of owns, a dotted ≠ between ?y and ?x, a red octagon of not pointing at the pattern of likes in the frame not ctx](renders/patterns.rules.03-rule_owns_unloved_lhs_rhs.svg)

- `cool-owner`, an `(absent …)`: its pattern drawn inside the cluster `absent (∄)`
  ([`renders/patterns.rules.04-rule_cool_owner_lhs_rhs.svg`](renders/patterns.rules.04-rule_cool_owner_lhs_rhs.svg));
- `believed`, a pattern under a pattern's arrow: `(owns ?p ?x)` its own octagon in the frame `claims ctx`, pointed
  at by `claims`'s arrow `3`
  ([`renders/patterns.rules.05-rule_believed_lhs_rhs.svg`](renders/patterns.rules.05-rule_believed_lhs_rhs.svg)).

Before S1f.23.1 (`build/review-bin/ein-head-b5fb8d70`, `renders/patterns.rules.before-b5fb8d70.*`) a pattern of
three arrows was a labelled arrow (`owns` from `?p` to `?x`), one of another count an octagon labelled with its
relation, `(not P)` a labelled arrow `¬likes`, and `(claims ?c (owns ?p ?x))` an arrow `claims` into an octagon
labelled `owns`.

**`forall` and `unknown`**, as their tests call them. [`programs/01_forall.ein`](programs/01_forall.ein)'s
`complete-via-macro`: `(forall ?y (member ?y ?g) (marked ?y))` an octagon with four arrows, `1` at the box `forall`,
`2` at `?y`, `3` and `4` at the two patterns in the frame `forall ctx`:

![complete-via-macro today: the pattern of forall an octagon whose arrow 1 is at the box forall and whose arrows 3 and 4 are at the patterns of member and marked in the frame forall ctx](renders/forall.rules.01-rule_complete_via_macro_lhs_rhs.svg)

and before, the cluster `forall (∀)` holding `?y`, a labelled arrow `member` and an octagon `marked`
([`renders/forall.rules.before-b5fb8d70.01-rule_complete_via_macro_lhs_rhs.svg`](renders/forall.rules.before-b5fb8d70.01-rule_complete_via_macro_lhs_rhs.svg)).
[`programs/02_unknown.ein`](programs/02_unknown.ein)'s `third-state`: `(unknown (likes ?p ?f))` an octagon at the box
`unknown`, its arrow `2` at the pattern of `likes` in `unknown ctx`
([`renders/unknown.rule-third-state.svg`](renders/unknown.rule-third-state.svg)).

**R-15: `domain-elimination`** ([`programs/bijection.ein`](programs/bijection.ein), a copy of `stdlib/bijection.ein`;
the same drawing as the reasoning page's `02-domain-elimination.dot`, byte for byte but its comment lines):

```lisp
(forall ?b_other
  (and (?isa ?b_other ?B) (neq ?b_other ?b))
  (not (?R ?a ?b_other)))
```

is drawn as the octagon of `forall` (arrows `1` `forall`, `2` `?b_other`, `3` the `and`, `4` the `not`), an octagon
of **`and`** in `forall ctx` whose arrows `2` and `3` are at an octagon of `?isa` and an octagon of **`neq`**, both in
the frame `and ctx`, and the `not` pointing at `(?R ?a ?b_other)` in `not ctx`: 29 nodes, 12 octagons, 3 frames, no
dotted link ([`runs/domain-elimination.counts.tsv`](runs/domain-elimination.counts.tsv)).

![domain-elimination today: the octagon of forall, under it an octagon of and in the frame forall ctx, whose arrows go to an octagon of ?isa and an octagon of neq in the frame and ctx; boxes and and neq beside the box forall](renders/domain-elimination.svg)

Before ([`renders/domain-elimination.before-b5fb8d70.svg`](renders/domain-elimination.before-b5fb8d70.svg)): 11 nodes,
the cluster `forall (∀)` holding `?b_other`, the `neq` the dotted `≠`, the negation a labelled arrow `¬?R`.

**A real program's rules**: zebra2's fifteen ([`programs/zebra2.ein`](programs/zebra2.ein), a copy of
`examples/zebra2.ein`), `renders/zebra2.rules.NN-*.svg` and `renders/zebra2.rules.fold.NN-*.svg` (§ 5).

## 3. The options

### (a) A pattern drawn as a fact; no macro read by its name (the default, built)

As above: `forall` and `unknown` are patterns as called, and their arguments are drawn as the forms they are
written as, so a `forall`'s guard draws `and` and `neq` as nested facts (R-15).

### (b) `forall` kept as a cluster of its own

The view reads `std.macro`'s `forall` by its name and draws a call of it as a cluster `forall (∀)` holding its
variable, its guard drawn as a match is (`and` read flat, `neq` the dotted link) and its body. That is reading a
theory's name in the renderer, which your ruling on theory names (Q-M1f.436) forbids; R-4 was this read, found by the
renaming test. Needs a code change: **mocks**, today's DOT edited:

- [`renders/forall.complete-via-macro.option-b.MOCK.dot`](renders/forall.complete-via-macro.option-b.MOCK.dot) /
  [`.svg`](renders/forall.complete-via-macro.option-b.MOCK.svg): the pattern of `forall` (its octagon, its four arrows,
  the box `forall`) deleted, its frame `forall ctx` renamed a cluster `forall (∀)` that also holds `?y`;
- [`renders/domain-elimination.option-b.MOCK.dot`](renders/domain-elimination.option-b.MOCK.dot) /
  [`.svg`](renders/domain-elimination.option-b.MOCK.svg): the same, and inside the cluster the guard as a match is:
  the octagon of `and` and the box `and` deleted, the frame `and ctx` gone, the octagon of `neq` and the box `neq`
  deleted and the dotted `≠` link drawn between `?b_other` and `?b`.

![MOCK, option (b): domain-elimination with a cluster forall (∀) holding ?b_other, the pattern of ?isa and the red octagon of not; the ≠ a dotted link to ?b](renders/domain-elimination.option-b.MOCK.svg)

### R-15's readings (the user's; each changes only how a macro's arguments are drawn)

- **(R-15 i) as (a)**: the arguments drawn as written, `and` and `neq` as nested facts in frames. Nothing to build.
- **(R-15 ii) the kernel's words keep their marks under any arrow**: `forall` stays a pattern, but an `and` under its
  arrow is read flat (its conjuncts under that arrow) and a `neq` is the dotted link, as in a match. Reads the
  kernel's words, not a theory's. **A mock**,
  [`renders/domain-elimination.r15-kernel-words.MOCK.dot`](renders/domain-elimination.r15-kernel-words.MOCK.dot) /
  [`.svg`](renders/domain-elimination.r15-kernel-words.MOCK.svg): today's DOT with the octagon of `and`, its arrows
  and the box `and` deleted, the frame `and ctx` gone and `forall`'s arrow `3` at `(?isa ?b_other ?B)`, now in
  `forall ctx`; the octagon of `neq` and the box `neq` deleted and the dotted `≠` link drawn.

  ![MOCK, R-15 (ii): forall still an octagon at the box forall; its arrow 3 at the pattern of ?isa in forall ctx, the ≠ a dotted link between ?b and ?b_other; no octagon of and or neq](renders/domain-elimination.r15-kernel-words.MOCK.svg)

- **(R-15 iii) a macro's call drawn as its expansion**: the loader expands `(forall ?b G B)` to
  `(absent (and G (absent B)))`, which reads only the kernel's `absent`, `and` and `neq`. Needs a code change (the
  view draws the parsed form). Its look is the binary's own drawing of the rule written out by hand,
  [`programs/de_expanded.ein`](programs/de_expanded.ein) (it loads, [`runs/de_expanded.solve.txt`](runs/de_expanded.solve.txt)):
  [`renders/domain-elimination-written-out.svg`](renders/domain-elimination-written-out.svg)
  ([dot](renders/domain-elimination-written-out.dot)): one cluster `absent (∄)`, the dotted `≠`, 23 nodes. But see
  *Found while building*: the inner `absent` is not drawn as one.

  ![the binary's drawing of domain-elimination written out: a cluster absent holding ?b_other, the pattern of ?isa, the ≠ link, a red octagon of not pointing at the pattern of ?R in not ctx](renders/domain-elimination-written-out.svg)

- **(R-15 iv) `forall` as a cluster**: option (b) above.

## 4. The default

**(a)**, taken by S1f.23.1: a pattern as a fact, an octagon with no name in it, its arrow `1` at its relation, a box
per panel, or the variable's diamond; folded under `--fold` where it has three arrows and nothing points at it; guards
keep their dotted links and `absent` its cluster; `(not P)` a red pattern of `not`, its arrow `2` at `P`'s node in
`not ctx` where `P` is no premise; no macro's name read: `forall` and `unknown` patterns as called; `()` draws
nothing. Built in `ein-render/src/rules.rs`; held by `every_object_once.rs`' `the_rule_view_draws_a_pattern_as_a_fact`
and `presentation_semantics.rs`. R-15 has no default: the user's.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | re-takes every run and drawing |
| [`programs/01_forall.ein`](programs/01_forall.ein) | a copy of `tests/stdlib/macro/01_forall.ein` |
| [`programs/02_unknown.ein`](programs/02_unknown.ein) | a copy of `tests/stdlib/macro/02_unknown.ein` |
| [`programs/bijection.ein`](programs/bijection.ein) | a copy of `stdlib/bijection.ein`, `std.bijection`, drawn as its own file |
| [`programs/de_expanded.ein`](programs/de_expanded.ein) | `domain-elimination` with `forall` written out as the macro expands it, written for this item |
| [`programs/de_one_absent.ein`](programs/de_one_absent.ein) | the same with one `absent` where the expansion has two, written for this item |
| [`programs/patterns.ein`](programs/patterns.ein) | one rule a pattern kind, written for this item |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | every command and its exit code |
| [`runs/de_expanded.solve.txt`](runs/de_expanded.solve.txt) | `ein solve programs/de_expanded.ein`: it loads |
| [`runs/domain-elimination.counts.tsv`](runs/domain-elimination.counts.tsv) | nodes by shape, frames and dotted links of four drawings |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds, a drawing |
| [`runs/one-absent-vs-two.diff`](runs/one-absent-vs-two.diff) | the two written-out drawings, diffed: none, the rule's name aside |
| [`renders/de-one-absent.dot`](renders/de-one-absent.dot) / [`.svg`](renders/de-one-absent.svg) | `ein render rule` of de_one_absent.ein |
| [`renders/domain-elimination-written-out.dot`](renders/domain-elimination-written-out.dot) / [`.svg`](renders/domain-elimination-written-out.svg) | `ein render rule` of de_expanded.ein |
| [`renders/domain-elimination.before-b5fb8d70.dot`](renders/domain-elimination.before-b5fb8d70.dot) / [`.svg`](renders/domain-elimination.before-b5fb8d70.svg) | by the binary before S1f.23.1 |
| [`renders/domain-elimination.dot`](renders/domain-elimination.dot) / [`.svg`](renders/domain-elimination.svg) | `ein render rule --name domain-elimination` of bijection.ein |
| [`renders/domain-elimination.fold.dot`](renders/domain-elimination.fold.dot) / [`.svg`](renders/domain-elimination.fold.svg) | the same, `--fold` |
| [`renders/domain-elimination.option-b.MOCK.dot`](renders/domain-elimination.option-b.MOCK.dot) / [`.svg`](renders/domain-elimination.option-b.MOCK.svg) | **mock**, see § 3 |
| [`renders/domain-elimination.overlay.dot`](renders/domain-elimination.overlay.dot) / [`.svg`](renders/domain-elimination.overlay.svg) | the same, `--rule-mode overlay` |
| [`renders/domain-elimination.overlay.fold.dot`](renders/domain-elimination.overlay.fold.dot) / [`.svg`](renders/domain-elimination.overlay.fold.svg) | `ein render rule --name domain-elimination --rule-mode overlay --fold` of bijection.ein |
| [`renders/domain-elimination.r15-kernel-words.MOCK.dot`](renders/domain-elimination.r15-kernel-words.MOCK.dot) / [`.svg`](renders/domain-elimination.r15-kernel-words.MOCK.svg) | **mock**, see § 3 |
| [`renders/forall.complete-via-macro.option-b.MOCK.dot`](renders/forall.complete-via-macro.option-b.MOCK.dot) / [`.svg`](renders/forall.complete-via-macro.option-b.MOCK.svg) | **mock**, see § 3 |
| [`renders/forall.rules.01-rule_complete_via_macro_lhs_rhs.dot`](renders/forall.rules.01-rule_complete_via_macro_lhs_rhs.dot) / [`.svg`](renders/forall.rules.01-rule_complete_via_macro_lhs_rhs.svg) | its part `rule_complete_via_macro_lhs_rhs` |
| [`renders/forall.rules.02-rule_complete_via_expansion_lhs_rhs.dot`](renders/forall.rules.02-rule_complete_via_expansion_lhs_rhs.dot) / [`.svg`](renders/forall.rules.02-rule_complete_via_expansion_lhs_rhs.svg) | its part `rule_complete_via_expansion_lhs_rhs` |
| [`renders/forall.rules.before-b5fb8d70.01-rule_complete_via_macro_lhs_rhs.dot`](renders/forall.rules.before-b5fb8d70.01-rule_complete_via_macro_lhs_rhs.dot) / [`.svg`](renders/forall.rules.before-b5fb8d70.01-rule_complete_via_macro_lhs_rhs.svg) | its part `rule_complete_via_macro_lhs_rhs`, before |
| [`renders/forall.rules.before-b5fb8d70.02-rule_complete_via_expansion_lhs_rhs.dot`](renders/forall.rules.before-b5fb8d70.02-rule_complete_via_expansion_lhs_rhs.dot) / [`.svg`](renders/forall.rules.before-b5fb8d70.02-rule_complete_via_expansion_lhs_rhs.svg) | its part `rule_complete_via_expansion_lhs_rhs`, before |
| [`renders/forall.rules.before-b5fb8d70.dot`](renders/forall.rules.before-b5fb8d70.dot) | by the binary before S1f.23.1 |
| [`renders/forall.rules.dot`](renders/forall.rules.dot) | `ein render rules` of 01_forall.ein, two digraphs |
| [`renders/forall.rules.fold.01-rule_complete_via_macro_lhs_rhs.dot`](renders/forall.rules.fold.01-rule_complete_via_macro_lhs_rhs.dot) / [`.svg`](renders/forall.rules.fold.01-rule_complete_via_macro_lhs_rhs.svg) | its part `rule_complete_via_macro_lhs_rhs`, folded |
| [`renders/forall.rules.fold.02-rule_complete_via_expansion_lhs_rhs.dot`](renders/forall.rules.fold.02-rule_complete_via_expansion_lhs_rhs.dot) / [`.svg`](renders/forall.rules.fold.02-rule_complete_via_expansion_lhs_rhs.svg) | its part `rule_complete_via_expansion_lhs_rhs`, folded |
| [`renders/forall.rules.fold.dot`](renders/forall.rules.fold.dot) | `ein render rules --fold` of 01_forall.ein |
| [`renders/forall.rules.overlay.01-rule_complete_via_macro_overlay.dot`](renders/forall.rules.overlay.01-rule_complete_via_macro_overlay.dot) / [`.svg`](renders/forall.rules.overlay.01-rule_complete_via_macro_overlay.svg) | its part `rule_complete_via_macro_overlay`, overlay |
| [`renders/forall.rules.overlay.02-rule_complete_via_expansion_overlay.dot`](renders/forall.rules.overlay.02-rule_complete_via_expansion_overlay.dot) / [`.svg`](renders/forall.rules.overlay.02-rule_complete_via_expansion_overlay.svg) | its part `rule_complete_via_expansion_overlay`, overlay |
| [`renders/forall.rules.overlay.dot`](renders/forall.rules.overlay.dot) | `ein render rules --rule-mode overlay` of 01_forall.ein |
| [`renders/forall.rules.overlay.fold.01-rule_complete_via_macro_overlay.dot`](renders/forall.rules.overlay.fold.01-rule_complete_via_macro_overlay.dot) / [`.svg`](renders/forall.rules.overlay.fold.01-rule_complete_via_macro_overlay.svg) | its part `rule_complete_via_macro_overlay`, overlay, folded |
| [`renders/forall.rules.overlay.fold.02-rule_complete_via_expansion_overlay.dot`](renders/forall.rules.overlay.fold.02-rule_complete_via_expansion_overlay.dot) / [`.svg`](renders/forall.rules.overlay.fold.02-rule_complete_via_expansion_overlay.svg) | its part `rule_complete_via_expansion_overlay`, overlay, folded |
| [`renders/forall.rules.overlay.fold.dot`](renders/forall.rules.overlay.fold.dot) | `ein render rules --rule-mode overlay --fold` of 01_forall.ein |
| [`renders/patterns.rules.01-rule_keeps_lhs_rhs.dot`](renders/patterns.rules.01-rule_keeps_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.01-rule_keeps_lhs_rhs.svg) | its part `rule_keeps_lhs_rhs` |
| [`renders/patterns.rules.02-rule_symmetric_lhs_rhs.dot`](renders/patterns.rules.02-rule_symmetric_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.02-rule_symmetric_lhs_rhs.svg) | its part `rule_symmetric_lhs_rhs` |
| [`renders/patterns.rules.03-rule_owns_unloved_lhs_rhs.dot`](renders/patterns.rules.03-rule_owns_unloved_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.03-rule_owns_unloved_lhs_rhs.svg) | its part `rule_owns_unloved_lhs_rhs` |
| [`renders/patterns.rules.04-rule_cool_owner_lhs_rhs.dot`](renders/patterns.rules.04-rule_cool_owner_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.04-rule_cool_owner_lhs_rhs.svg) | its part `rule_cool_owner_lhs_rhs` |
| [`renders/patterns.rules.05-rule_believed_lhs_rhs.dot`](renders/patterns.rules.05-rule_believed_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.05-rule_believed_lhs_rhs.svg) | its part `rule_believed_lhs_rhs` |
| [`renders/patterns.rules.before-b5fb8d70.01-rule_keeps_lhs_rhs.dot`](renders/patterns.rules.before-b5fb8d70.01-rule_keeps_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.before-b5fb8d70.01-rule_keeps_lhs_rhs.svg) | its part `rule_keeps_lhs_rhs`, before |
| [`renders/patterns.rules.before-b5fb8d70.02-rule_symmetric_lhs_rhs.dot`](renders/patterns.rules.before-b5fb8d70.02-rule_symmetric_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.before-b5fb8d70.02-rule_symmetric_lhs_rhs.svg) | its part `rule_symmetric_lhs_rhs`, before |
| [`renders/patterns.rules.before-b5fb8d70.03-rule_owns_unloved_lhs_rhs.dot`](renders/patterns.rules.before-b5fb8d70.03-rule_owns_unloved_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.before-b5fb8d70.03-rule_owns_unloved_lhs_rhs.svg) | its part `rule_owns_unloved_lhs_rhs`, before |
| [`renders/patterns.rules.before-b5fb8d70.04-rule_cool_owner_lhs_rhs.dot`](renders/patterns.rules.before-b5fb8d70.04-rule_cool_owner_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.before-b5fb8d70.04-rule_cool_owner_lhs_rhs.svg) | its part `rule_cool_owner_lhs_rhs`, before |
| [`renders/patterns.rules.before-b5fb8d70.05-rule_believed_lhs_rhs.dot`](renders/patterns.rules.before-b5fb8d70.05-rule_believed_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.before-b5fb8d70.05-rule_believed_lhs_rhs.svg) | its part `rule_believed_lhs_rhs`, before |
| [`renders/patterns.rules.before-b5fb8d70.dot`](renders/patterns.rules.before-b5fb8d70.dot) | by the binary before S1f.23.1 |
| [`renders/patterns.rules.dot`](renders/patterns.rules.dot) | `ein render rules` of patterns.ein, side by side, five digraphs |
| [`renders/patterns.rules.fold.01-rule_keeps_lhs_rhs.dot`](renders/patterns.rules.fold.01-rule_keeps_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.fold.01-rule_keeps_lhs_rhs.svg) | its part `rule_keeps_lhs_rhs`, folded |
| [`renders/patterns.rules.fold.02-rule_symmetric_lhs_rhs.dot`](renders/patterns.rules.fold.02-rule_symmetric_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.fold.02-rule_symmetric_lhs_rhs.svg) | its part `rule_symmetric_lhs_rhs`, folded |
| [`renders/patterns.rules.fold.03-rule_owns_unloved_lhs_rhs.dot`](renders/patterns.rules.fold.03-rule_owns_unloved_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.fold.03-rule_owns_unloved_lhs_rhs.svg) | its part `rule_owns_unloved_lhs_rhs`, folded |
| [`renders/patterns.rules.fold.04-rule_cool_owner_lhs_rhs.dot`](renders/patterns.rules.fold.04-rule_cool_owner_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.fold.04-rule_cool_owner_lhs_rhs.svg) | its part `rule_cool_owner_lhs_rhs`, folded |
| [`renders/patterns.rules.fold.05-rule_believed_lhs_rhs.dot`](renders/patterns.rules.fold.05-rule_believed_lhs_rhs.dot) / [`.svg`](renders/patterns.rules.fold.05-rule_believed_lhs_rhs.svg) | its part `rule_believed_lhs_rhs`, folded |
| [`renders/patterns.rules.fold.dot`](renders/patterns.rules.fold.dot) | `ein render rules --fold` of patterns.ein, five digraphs |
| [`renders/patterns.rules.overlay.01-rule_keeps_overlay.dot`](renders/patterns.rules.overlay.01-rule_keeps_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.01-rule_keeps_overlay.svg) | its part `rule_keeps_overlay`, overlay |
| [`renders/patterns.rules.overlay.02-rule_symmetric_overlay.dot`](renders/patterns.rules.overlay.02-rule_symmetric_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.02-rule_symmetric_overlay.svg) | its part `rule_symmetric_overlay`, overlay |
| [`renders/patterns.rules.overlay.03-rule_owns_unloved_overlay.dot`](renders/patterns.rules.overlay.03-rule_owns_unloved_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.03-rule_owns_unloved_overlay.svg) | its part `rule_owns_unloved_overlay`, overlay |
| [`renders/patterns.rules.overlay.04-rule_cool_owner_overlay.dot`](renders/patterns.rules.overlay.04-rule_cool_owner_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.04-rule_cool_owner_overlay.svg) | its part `rule_cool_owner_overlay`, overlay |
| [`renders/patterns.rules.overlay.05-rule_believed_overlay.dot`](renders/patterns.rules.overlay.05-rule_believed_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.05-rule_believed_overlay.svg) | its part `rule_believed_overlay`, overlay |
| [`renders/patterns.rules.overlay.dot`](renders/patterns.rules.overlay.dot) | `ein render rules --rule-mode overlay` of patterns.ein, five digraphs |
| [`renders/patterns.rules.overlay.fold.01-rule_keeps_overlay.dot`](renders/patterns.rules.overlay.fold.01-rule_keeps_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.fold.01-rule_keeps_overlay.svg) | its part `rule_keeps_overlay`, overlay, folded |
| [`renders/patterns.rules.overlay.fold.02-rule_symmetric_overlay.dot`](renders/patterns.rules.overlay.fold.02-rule_symmetric_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.fold.02-rule_symmetric_overlay.svg) | its part `rule_symmetric_overlay`, overlay, folded |
| [`renders/patterns.rules.overlay.fold.03-rule_owns_unloved_overlay.dot`](renders/patterns.rules.overlay.fold.03-rule_owns_unloved_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.fold.03-rule_owns_unloved_overlay.svg) | its part `rule_owns_unloved_overlay`, overlay, folded |
| [`renders/patterns.rules.overlay.fold.04-rule_cool_owner_overlay.dot`](renders/patterns.rules.overlay.fold.04-rule_cool_owner_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.fold.04-rule_cool_owner_overlay.svg) | its part `rule_cool_owner_overlay`, overlay, folded |
| [`renders/patterns.rules.overlay.fold.05-rule_believed_overlay.dot`](renders/patterns.rules.overlay.fold.05-rule_believed_overlay.dot) / [`.svg`](renders/patterns.rules.overlay.fold.05-rule_believed_overlay.svg) | its part `rule_believed_overlay`, overlay, folded |
| [`renders/patterns.rules.overlay.fold.dot`](renders/patterns.rules.overlay.fold.dot) | `ein render rules --rule-mode overlay --fold` of patterns.ein, five digraphs |
| [`renders/unknown.rule-third-state.dot`](renders/unknown.rule-third-state.dot) / [`.svg`](renders/unknown.rule-third-state.svg) | `ein render rule --name third-state` of 02_unknown.ein |
| [`renders/unknown.rule-third-state.fold.dot`](renders/unknown.rule-third-state.fold.dot) / [`.svg`](renders/unknown.rule-third-state.fold.svg) | the same, `--fold` |
| [`renders/unknown.rule-third-state.overlay.dot`](renders/unknown.rule-third-state.overlay.dot) / [`.svg`](renders/unknown.rule-third-state.overlay.svg) | the same, `--rule-mode overlay` |
| [`renders/unknown.rule-third-state.overlay.fold.dot`](renders/unknown.rule-third-state.overlay.fold.dot) / [`.svg`](renders/unknown.rule-third-state.overlay.fold.svg) | `ein render rule --name third-state --rule-mode overlay --fold` of 02_unknown.ein |
| [`renders/zebra2.rules.01-rule_co_located_lhs_rhs.dot`](renders/zebra2.rules.01-rule_co_located_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.01-rule_co_located_lhs_rhs.svg) | its part `rule_co_located_lhs_rhs` |
| [`renders/zebra2.rules.02-rule_co_located_fanout_lhs_rhs.dot`](renders/zebra2.rules.02-rule_co_located_fanout_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.02-rule_co_located_fanout_lhs_rhs.svg) | its part `rule_co_located_fanout_lhs_rhs` |
| [`renders/zebra2.rules.03-rule_co_located_negative_lhs_rhs.dot`](renders/zebra2.rules.03-rule_co_located_negative_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.03-rule_co_located_negative_lhs_rhs.svg) | its part `rule_co_located_negative_lhs_rhs` |
| [`renders/zebra2.rules.04-rule_adjacent_via_fwd_lhs_rhs.dot`](renders/zebra2.rules.04-rule_adjacent_via_fwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.04-rule_adjacent_via_fwd_lhs_rhs.svg) | its part `rule_adjacent_via_fwd_lhs_rhs` |
| [`renders/zebra2.rules.05-rule_adjacent_via_bwd_lhs_rhs.dot`](renders/zebra2.rules.05-rule_adjacent_via_bwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.05-rule_adjacent_via_bwd_lhs_rhs.svg) | its part `rule_adjacent_via_bwd_lhs_rhs` |
| [`renders/zebra2.rules.06-rule_adjacent_via_fwd_negative_lhs_rhs.dot`](renders/zebra2.rules.06-rule_adjacent_via_fwd_negative_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.06-rule_adjacent_via_fwd_negative_lhs_rhs.svg) | its part `rule_adjacent_via_fwd_negative_lhs_rhs` |
| [`renders/zebra2.rules.07-rule_adjacent_via_bwd_negative_lhs_rhs.dot`](renders/zebra2.rules.07-rule_adjacent_via_bwd_negative_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.07-rule_adjacent_via_bwd_negative_lhs_rhs.svg) | its part `rule_adjacent_via_bwd_negative_lhs_rhs` |
| [`renders/zebra2.rules.08-rule_derive_adjacent_via_lhs_rhs.dot`](renders/zebra2.rules.08-rule_derive_adjacent_via_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.08-rule_derive_adjacent_via_lhs_rhs.svg) | its part `rule_derive_adjacent_via_lhs_rhs` |
| [`renders/zebra2.rules.09-rule_derive_adjacent_via_fwd_fanout_lhs_rhs.dot`](renders/zebra2.rules.09-rule_derive_adjacent_via_fwd_fanout_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.09-rule_derive_adjacent_via_fwd_fanout_lhs_rhs.svg) | its part `rule_derive_adjacent_via_fwd_fanout_lhs_rhs` |
| [`renders/zebra2.rules.10-rule_derive_adjacent_via_bwd_fanout_lhs_rhs.dot`](renders/zebra2.rules.10-rule_derive_adjacent_via_bwd_fanout_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.10-rule_derive_adjacent_via_bwd_fanout_lhs_rhs.svg) | its part `rule_derive_adjacent_via_bwd_fanout_lhs_rhs` |
| [`renders/zebra2.rules.11-rule_disjunctive_prune_fwd_lhs_rhs.dot`](renders/zebra2.rules.11-rule_disjunctive_prune_fwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.11-rule_disjunctive_prune_fwd_lhs_rhs.svg) | its part `rule_disjunctive_prune_fwd_lhs_rhs` |
| [`renders/zebra2.rules.12-rule_disjunctive_prune_bwd_lhs_rhs.dot`](renders/zebra2.rules.12-rule_disjunctive_prune_bwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.12-rule_disjunctive_prune_bwd_lhs_rhs.svg) | its part `rule_disjunctive_prune_bwd_lhs_rhs` |
| [`renders/zebra2.rules.13-rule_adjacent_via_endpoint_fwd_lhs_rhs.dot`](renders/zebra2.rules.13-rule_adjacent_via_endpoint_fwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.13-rule_adjacent_via_endpoint_fwd_lhs_rhs.svg) | its part `rule_adjacent_via_endpoint_fwd_lhs_rhs` |
| [`renders/zebra2.rules.14-rule_adjacent_via_endpoint_bwd_lhs_rhs.dot`](renders/zebra2.rules.14-rule_adjacent_via_endpoint_bwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.14-rule_adjacent_via_endpoint_bwd_lhs_rhs.svg) | its part `rule_adjacent_via_endpoint_bwd_lhs_rhs` |
| [`renders/zebra2.rules.15-rule_guess_lhs_rhs.dot`](renders/zebra2.rules.15-rule_guess_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.15-rule_guess_lhs_rhs.svg) | its part `rule_guess_lhs_rhs` |
| [`renders/zebra2.rules.dot`](renders/zebra2.rules.dot) | `ein render rules` of zebra2, fifteen digraphs |
| [`renders/zebra2.rules.fold.01-rule_co_located_lhs_rhs.dot`](renders/zebra2.rules.fold.01-rule_co_located_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.01-rule_co_located_lhs_rhs.svg) | its part `rule_co_located_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.02-rule_co_located_fanout_lhs_rhs.dot`](renders/zebra2.rules.fold.02-rule_co_located_fanout_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.02-rule_co_located_fanout_lhs_rhs.svg) | its part `rule_co_located_fanout_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.03-rule_co_located_negative_lhs_rhs.dot`](renders/zebra2.rules.fold.03-rule_co_located_negative_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.03-rule_co_located_negative_lhs_rhs.svg) | its part `rule_co_located_negative_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.04-rule_adjacent_via_fwd_lhs_rhs.dot`](renders/zebra2.rules.fold.04-rule_adjacent_via_fwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.04-rule_adjacent_via_fwd_lhs_rhs.svg) | its part `rule_adjacent_via_fwd_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.05-rule_adjacent_via_bwd_lhs_rhs.dot`](renders/zebra2.rules.fold.05-rule_adjacent_via_bwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.05-rule_adjacent_via_bwd_lhs_rhs.svg) | its part `rule_adjacent_via_bwd_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.06-rule_adjacent_via_fwd_negative_lhs_rhs.dot`](renders/zebra2.rules.fold.06-rule_adjacent_via_fwd_negative_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.06-rule_adjacent_via_fwd_negative_lhs_rhs.svg) | its part `rule_adjacent_via_fwd_negative_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.07-rule_adjacent_via_bwd_negative_lhs_rhs.dot`](renders/zebra2.rules.fold.07-rule_adjacent_via_bwd_negative_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.07-rule_adjacent_via_bwd_negative_lhs_rhs.svg) | its part `rule_adjacent_via_bwd_negative_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.08-rule_derive_adjacent_via_lhs_rhs.dot`](renders/zebra2.rules.fold.08-rule_derive_adjacent_via_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.08-rule_derive_adjacent_via_lhs_rhs.svg) | its part `rule_derive_adjacent_via_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.09-rule_derive_adjacent_via_fwd_fanout_lhs_rhs.dot`](renders/zebra2.rules.fold.09-rule_derive_adjacent_via_fwd_fanout_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.09-rule_derive_adjacent_via_fwd_fanout_lhs_rhs.svg) | its part `rule_derive_adjacent_via_fwd_fanout_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.10-rule_derive_adjacent_via_bwd_fanout_lhs_rhs.dot`](renders/zebra2.rules.fold.10-rule_derive_adjacent_via_bwd_fanout_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.10-rule_derive_adjacent_via_bwd_fanout_lhs_rhs.svg) | its part `rule_derive_adjacent_via_bwd_fanout_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.11-rule_disjunctive_prune_fwd_lhs_rhs.dot`](renders/zebra2.rules.fold.11-rule_disjunctive_prune_fwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.11-rule_disjunctive_prune_fwd_lhs_rhs.svg) | its part `rule_disjunctive_prune_fwd_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.12-rule_disjunctive_prune_bwd_lhs_rhs.dot`](renders/zebra2.rules.fold.12-rule_disjunctive_prune_bwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.12-rule_disjunctive_prune_bwd_lhs_rhs.svg) | its part `rule_disjunctive_prune_bwd_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.13-rule_adjacent_via_endpoint_fwd_lhs_rhs.dot`](renders/zebra2.rules.fold.13-rule_adjacent_via_endpoint_fwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.13-rule_adjacent_via_endpoint_fwd_lhs_rhs.svg) | its part `rule_adjacent_via_endpoint_fwd_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.14-rule_adjacent_via_endpoint_bwd_lhs_rhs.dot`](renders/zebra2.rules.fold.14-rule_adjacent_via_endpoint_bwd_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.14-rule_adjacent_via_endpoint_bwd_lhs_rhs.svg) | its part `rule_adjacent_via_endpoint_bwd_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.15-rule_guess_lhs_rhs.dot`](renders/zebra2.rules.fold.15-rule_guess_lhs_rhs.dot) / [`.svg`](renders/zebra2.rules.fold.15-rule_guess_lhs_rhs.svg) | its part `rule_guess_lhs_rhs`, folded |
| [`renders/zebra2.rules.fold.dot`](renders/zebra2.rules.fold.dot) | `ein render rules --fold` of zebra2, fifteen digraphs |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/38-Q-M1f.427/rerun.sh
```

It draws every rule four ways with today's binary, three with `ein-head-b5fb8d70` for *before*, diffs the two
written-out rules, and draws the three mocks from their hand-edited DOT.

## 7. Found while building

- **A nested `absent` is not drawn as a cluster, and a rule with two `absent`s draws as one with one.** In
  `complete-via-expansion` (01_forall.ein), `(absent (and (member ?y ?g) (absent (marked ?y))))`, the inner
  `(absent (marked ?y))` is a red octagon of `marked` inside the outer cluster, no cluster of its own
  ([`renders/forall.rules.02-rule_complete_via_expansion_lhs_rhs.svg`](renders/forall.rules.02-rule_complete_via_expansion_lhs_rhs.svg));
  red is also `(not P)`'s colour. So `domain-elimination` written out, `(absent (and G (absent (not B))))`, and the
  same with one `absent`, `(absent (and G (not B)))` ([`programs/de_one_absent.ein`](programs/de_one_absent.ein)), two
  rules that mean different things, draw **byte for byte alike** but for the rule's name
  ([`runs/one-absent-vs-two.diff`](runs/one-absent-vs-two.diff), empty). `12-rendering.md` says an `(absent …)` is a
  cluster; a nested one is not. Not this question's: the rule view's, for the ledger.
- **A pattern's relation that is a variable folds into a word**: under `--fold`, `(?isa ?b ?B)` and `(?rel ?a ?b)`
  are arrows labelled `?isa`, `?rel`, so a variable can be a diamond in one pattern and a label in another
  ([`renders/domain-elimination.fold.svg`](renders/domain-elimination.fold.svg)). (Also item
  [36](../36-Q-M1f.425/README.md).)
- The rule view's node ids are the patterns' text (`"(?isa ?b_other ?B)_L"`), never a label: no node shows a fact's
  text, as the ruling asks, but the DOT does.
- The macro's guard in a frame named `and ctx` reads as if `and` were a relation that holds facts in a context.

## 8. Your call

- **(a)** a pattern drawn as a fact, no macro read by name, `forall`'s arguments as written (default, built).
- **(b)** `forall` kept as a cluster of its own (reads a theory's name).
- **R-15**: (i) as (a); (ii) `and` and `neq` under any arrow keep their marks in a match; (iii) a macro's call drawn
  as its expansion; (iv) = (b).
