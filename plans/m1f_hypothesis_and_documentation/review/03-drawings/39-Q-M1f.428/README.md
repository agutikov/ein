# Q-M1f.428 — how a derivation is drawn

## 1. The question

A derivation is drawn in three places: **a trace's slice** (`ein solve --trace`, one drawing a step: the facts the
step read, the rule's firing, the facts it derived), **the derivation graph** (a fact and everything it rests on), and
**a written trace's `dag` view** (a `(trace …)` form's steps, each from its premises to the fact it derives). Before
S1f.23.1 each drew a fact as a node labelled with its text, `friends(Bob, Carol)` (form 4, which your ruling forbids).
The build draws **each fact as its node, in the canonical form, and each step as a node of its own**, from its
premises to what it derives: in a slice, the firing's box; a commitment's facts red and a stored negation grey, no
node filled; a fact that a fact of the slice points at is its node too, framed where the state does not hold it; a
refutation's `⊥` a mark of the step that its core's facts point at, and the learned no-good a note pointing at each
of its facts. The derivation graph draws **a diamond for every derivation**, also where a fact has only one, and
what a fact rests on beside it. The other reading: the derivation graph's diamond only where a fact has two
derivations, as before.

Recorded: [`open_questions.md` § Q-M1f.428](../../../open_questions.md#q-m1f428--how-a-derivation-is-drawn) (put, not
ruled; default (a)). It answers L-99 (a node with a fact's text in a derivation's drawing).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; every slice laid out by `dot`, left to right.

**A trace's slice.** Step 1 of [`programs/small.ein`](programs/small.ein)'s root, `symmetric` from
`(friends Bob Carol)` to `(friends Carol Bob)` ([`runs/small.trace.md`](runs/small.trace.md)): two octagons, each
with its arrows `1` at `friends` and `2`, `3` at `Bob` and `Carol`, the derived one bold; the firing a rounded box
labelled with the rule and its sentence, from the premise's octagon to the conclusion's.

![step 1 today: the premise's octagon, the rule's box symmetric with its sentence, the derived fact's octagon in bold; both octagons point at friends, Bob and Carol](renders/small.trace.root.step1.svg)

Before S1f.23.1 (`build/review-bin/ein-head-b5fb8d70`,
[`renders/small.trace.before-b5fb8d70.root.step1.svg`](renders/small.trace.before-b5fb8d70.root.step1.svg)): the same
step, each fact a rounded box labelled `friends(Bob, Carol)`.

![step 1 before: three boxes, friends(Bob, Carol), the rule, friends(Carol, Bob)](renders/small.trace.before-b5fb8d70.root.step1.svg)

**A step that concludes `(false)`**: [`programs/dead.ein`](programs/dead.ein) (Bob owns two pets, and a rule says one
person owns one) is dead at root; its trace's step 1 ([`runs/dead.trace.md`](runs/dead.trace.md),
[`renders/dead.trace.root.step1.svg`](renders/dead.trace.root.step1.svg)) draws `(false)` as an octagon of one arrow
at the box `false`, bold. Its **N** section is text, *Core: `(owns Bob Cat)`, `(owns Bob Dog)` · Refuted by
`one-pet`*: no drawing.

**A state's step**: [`programs/since.ein`](programs/since.ein), state 2, committed to `(owns Bob Dog)`
([`runs/since.trace.md`](runs/since.trace.md), [`renders/since.trace.state2.step1.svg`](renders/since.trace.state2.step1.svg)):
the commitment's fact is a premise of `pet-owner`, drawn black like any premise, not red (see *Found while building*).

**A real program**: [`programs/add.ein`](programs/add.ein), the calculator's sum, 60 steps
([`runs/add.trace.md`](runs/add.trace.md); every slice cut to `runs/add.trace.slices/`). Three drawn:
[step 7](renders/add.trace.root.step7.svg) (`add-ask`), [step 38](renders/add.trace.root.step38.svg) (`add-end`) and
[step 41](renders/add.trace.root.step41.svg) (`add-column`, *3 + 4 + carry 0 is 7*): 18 octagons, every numeral `(n …
d)` its own octagon, in six frames named by the module's relations in full,
`std.math.decimal-arith.add-walk, std.math.decimal-arith.b ctx`.

![step 41 of the add trace: the firing add-column from two octagons to a bold one; eighteen octagons, the numerals nested under the walk's facts, six dashed frames with long module names](renders/add.trace.root.step41.svg)

**The derivation graph** has no command (`ein-render`'s `derivation_dag_to_dot`, called only by its test,
[`runs/who-draws-what.txt`](runs/who-draws-what.txt)). Its golden, copied, read, not run:
[`renders/golden.kb_provenance_dag.dot`](renders/golden.kb_provenance_dag.dot) /
[`.svg`](renders/golden.kb_provenance_dag.svg): `(q a c)` derived by `triangle` from `(p a b)` and `(p b c)`, each
fact its octagon with its note beside it (`by triangle`, `(1)`, `(2)`), and **one diamond** for the one derivation,
the premises' bold arrows into it and its bold arrow to the conclusion.

![the derivation graph's golden: three octagons, a small diamond between the two premises and the conclusion, by triangle, (1) and (2) beside them](renders/golden.kb_provenance_dag.svg)

**A written trace's `dag` view** has no command either: the CLI draws a `(trace …)` form in its steps view, and the
`dag` view is chosen by a name no crate's command parses ([`runs/who-draws-what.txt`](runs/who-draws-what.txt)). Its
golden, read, not run: [`renders/golden.ir_render_trace_dag.svg`](renders/golden.ir_render_trace_dag.svg)
([dot](renders/golden.ir_render_trace_dag.dot)): each step a rounded box `s1 / from-condition`, the fact it derives an
octagon. What `ein render program` draws of [`programs/trace-steps.ein`](programs/trace-steps.ein) is the steps
view ([`renders/trace-steps.program.svg`](renders/trace-steps.program.svg)): boxes for the steps and the names they
use, and **no fact**: `(lives-in N H1)` and `(color-loc Blue H2)`, which `s1` and `s2` derive, are not drawn.

![ein render program of a written trace: boxes c10, c15, s1, s2, step: s3, step: s4 and dashed arrows; no fact drawn](renders/trace-steps.program.svg)

**A slice with a commitment and negations**: the slice's golden, copied
([`renders/golden.slice_render_slice.svg`](renders/golden.slice_render_slice.svg)): a commitment's fact red, two
stored negations grey, three firings. No trace step draws a red fact (see *Found while building*).

## 3. The options

### (a) Each fact its node, each step a node of its own; a diamond for every derivation (the default, built)

As above.

### (b) The derivation graph's diamond only where a fact has two derivations

As before S1f.23.1: one derivation is drawn as the premises' arrows straight into the conclusion, and the diamonds
appear only where a fact has alternatives. Needs a code change: **a mock**,
[`renders/derivation-graph.option-b.MOCK.dot`](renders/derivation-graph.option-b.MOCK.dot) /
[`.svg`](renders/derivation-graph.option-b.MOCK.svg): the golden with its diamond `j0` deleted and each premise's arrow
drawn to the conclusion. Nothing else edited. The slices are the same under (a) and (b).

![MOCK, option (b): the two premises' bold arrows straight into the conclusion's octagon, no diamond](renders/derivation-graph.option-b.MOCK.svg)

## 4. The default

**(a)**, taken by S1f.23.1: each fact as its node, each step a node of its own; in a slice a commitment's facts red, a
stored negation grey, no node filled; `⊥` a mark of the step; the learned no-good a note; the derivation graph a
diamond for every derivation, and what a fact rests on beside it; a written trace's `dag` view each step a node from
its premises to the fact it derives. Built in `ein-render/src/slice.rs`, `derivation.rs`, `ir_dot.rs`'
`render_trace_dag`; the goldens `slice_render_slice.dot`, `kb_provenance_dag.dot`, `ir_render_trace_dag.dot`.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | re-takes every run and drawing |
| [`programs/add.ein`](programs/add.ein) | a copy of `examples/arith/add.ein` |
| [`programs/dead.ein`](programs/dead.ein) | a program whose root is dead, written for this item |
| [`programs/since.ein`](programs/since.ein) | two states, written for the drawings' review |
| [`programs/small.ein`](programs/small.ein) | the small program, written for the drawings' review |
| [`programs/trace-steps.ein`](programs/trace-steps.ein) | a copy of `examples/syntax/trace-steps.ein`, a written `(trace …)` |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | every command and its exit code |
| [`runs/add.trace.md`](runs/add.trace.md) | `ein solve programs/add.ein --trace`, the region trace |
| [`runs/add.trace.stdout.txt`](runs/add.trace.stdout.txt) | its read-out |
| [`runs/dead.trace.md`](runs/dead.trace.md) | `ein solve programs/dead.ein --trace`, the region trace |
| [`runs/dead.trace.stdout.txt`](runs/dead.trace.stdout.txt) | its read-out |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds, a drawing |
| [`runs/since.trace.md`](runs/since.trace.md) | `ein solve programs/since.ein --trace`, the region trace |
| [`runs/since.trace.stdout.txt`](runs/since.trace.stdout.txt) | its read-out |
| [`runs/small.trace.before-b5fb8d70.md`](runs/small.trace.before-b5fb8d70.md) | `ein solve --trace` of small.ein by the binary before S1f.23.1 |
| [`runs/small.trace.before-b5fb8d70.stdout.txt`](runs/small.trace.before-b5fb8d70.stdout.txt) | its read-out |
| [`runs/small.trace.md`](runs/small.trace.md) | `ein solve programs/small.ein --trace`, the region trace |
| [`runs/small.trace.stdout.txt`](runs/small.trace.stdout.txt) | its read-out |
| [`runs/who-draws-what.txt`](runs/who-draws-what.txt) | what reaches the derivation graph, the `dag` view and a slice's `⊥` (read, not run) |
| [`renders/add.trace.root.step38.dot`](renders/add.trace.root.step38.dot) / [`.svg`](renders/add.trace.root.step38.svg) | step38 of the root section's slices |
| [`renders/add.trace.root.step41.dot`](renders/add.trace.root.step41.dot) / [`.svg`](renders/add.trace.root.step41.svg) | step41 of the root section's slices |
| [`renders/add.trace.root.step7.dot`](renders/add.trace.root.step7.dot) / [`.svg`](renders/add.trace.root.step7.svg) | step7 of the root section's slices |
| [`renders/dead.trace.root.step1.dot`](renders/dead.trace.root.step1.dot) / [`.svg`](renders/dead.trace.root.step1.svg) | step1 of the root section's slices |
| [`renders/dead.trace.root.step2.dot`](renders/dead.trace.root.step2.dot) / [`.svg`](renders/dead.trace.root.step2.svg) | step2 of the root section's slices |
| [`renders/derivation-graph.option-b.MOCK.dot`](renders/derivation-graph.option-b.MOCK.dot) / [`.svg`](renders/derivation-graph.option-b.MOCK.svg) | **mock**, option (b) |
| [`renders/golden.ir_render_trace_dag.dot`](renders/golden.ir_render_trace_dag.dot) / [`.svg`](renders/golden.ir_render_trace_dag.svg) | a written trace's `dag` view's golden, copied (read, not run) |
| [`renders/golden.kb_provenance_dag.dot`](renders/golden.kb_provenance_dag.dot) / [`.svg`](renders/golden.kb_provenance_dag.svg) | the derivation graph's golden, `ein-render/tests/golden/dot/`, copied (read, not run) |
| [`renders/golden.slice_render_slice.dot`](renders/golden.slice_render_slice.dot) / [`.svg`](renders/golden.slice_render_slice.svg) | a slice's golden, with a commitment's fact red and two negations grey, copied (read, not run) |
| [`renders/since.trace.root.step1.dot`](renders/since.trace.root.step1.dot) / [`.svg`](renders/since.trace.root.step1.svg) | step1 of the root section's slices |
| [`renders/since.trace.state2.step1.dot`](renders/since.trace.state2.step1.dot) / [`.svg`](renders/since.trace.state2.step1.svg) | step1 of the state2 section's slices |
| [`renders/small.trace.before-b5fb8d70.root.step1.dot`](renders/small.trace.before-b5fb8d70.root.step1.dot) / [`.svg`](renders/small.trace.before-b5fb8d70.root.step1.svg) | step1 of the small program's trace, before S1f.23.1 |
| [`renders/small.trace.before-b5fb8d70.root.step2.dot`](renders/small.trace.before-b5fb8d70.root.step2.dot) / [`.svg`](renders/small.trace.before-b5fb8d70.root.step2.svg) | step2 of the small program's trace, before S1f.23.1 |
| [`renders/small.trace.before-b5fb8d70.root.step3.dot`](renders/small.trace.before-b5fb8d70.root.step3.dot) / [`.svg`](renders/small.trace.before-b5fb8d70.root.step3.svg) | step3 of the small program's trace, before S1f.23.1 |
| [`renders/small.trace.before-b5fb8d70.root.step4.dot`](renders/small.trace.before-b5fb8d70.root.step4.dot) / [`.svg`](renders/small.trace.before-b5fb8d70.root.step4.svg) | step4 of the small program's trace, before S1f.23.1 |
| [`renders/small.trace.root.step1.dot`](renders/small.trace.root.step1.dot) / [`.svg`](renders/small.trace.root.step1.svg) | step1 of the root section's slices |
| [`renders/small.trace.root.step2.dot`](renders/small.trace.root.step2.dot) / [`.svg`](renders/small.trace.root.step2.svg) | step2 of the root section's slices |
| [`renders/small.trace.root.step3.dot`](renders/small.trace.root.step3.dot) / [`.svg`](renders/small.trace.root.step3.svg) | step3 of the root section's slices |
| [`renders/small.trace.root.step4.dot`](renders/small.trace.root.step4.dot) / [`.svg`](renders/small.trace.root.step4.svg) | step4 of the root section's slices |
| [`renders/trace-steps.program.dot`](renders/trace-steps.program.dot) / [`.svg`](renders/trace-steps.program.svg) | `ein render program` of trace-steps.ein: the steps view |
| `runs/add.trace.slices/add.trace.root.step1.dot` … `step60.dot` | every slice of the add trace, cut from it (sixty, not drawn; three are copied to `renders/`) |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/39-Q-M1f.428/rerun.sh
```

It traces four programs with today's binary and one with `ein-head-b5fb8d70`, cuts the slices, draws the program view
of the written trace, copies three goldens, draws everything, and writes `runs/who-draws-what.txt`.

## 7. Found while building

- **Three of the drawings this question rules have no command.** The derivation graph (`derivation_dag_to_dot`), a
  written trace's `dag` view (`TraceView::Dag`, which the CLI never chooses), and a slice's `⊥` with its learned
  no-good (`render_slice`'s `contradiction`, which the trace passes as `None`): each is reached by `ein-render`'s
  tests alone ([`runs/who-draws-what.txt`](runs/who-draws-what.txt)). [Q-M1e.20](../../../../../docs/decisions/cli/Q-M1e.20-every-renderer-has-a-command-that-reaches-it.md)
  says *no renderer the engine builds is reachable from Rust alone*. A dead commitment in a trace is told in text
  (*Core …, Refuted by …*), with no drawing.
- **No trace step draws a commitment's fact red.** The trace calls `render_slice` with an empty commitment
  (`linearize.rs`, read), so in state 2 of since.ein the committed `(owns Bob Dog)` is drawn black; only the golden's
  fixture has a red fact. `12-rendering.md` § The trace says *a commitment's facts are red*.
- **`ein render program` draws a written trace without its facts**: the steps view draws each step and each name a
  step uses as a box, and none of the facts the steps derive; and a step that a later step names (`s1`, `s2`) loses
  its label `step: s1`, drawn as a bare name like a premise.
- add.ein's slices name the module's relations in full in their frames (`std.math.decimal-arith.add-walk ctx`), since
  the program lists none of them; the frames are wider than their facts.

## 8. Your call

- **(a)** each fact its node, each step a node of its own, a diamond for every derivation (default, built).
- **(b)** the derivation graph's diamond only where a fact has two derivations.
