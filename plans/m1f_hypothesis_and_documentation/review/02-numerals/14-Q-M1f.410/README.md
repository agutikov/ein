# Q-M1f.410 — no join on a number, and a result a program's rule hands on

## 1. The question

The calculator keeps a standing rule: **no rule joins two facts on a number** (the engine answers such a join by a
scan, Q-M1f.362). The check keeps it: the walk carries the numeral it checks, so its end labels it with no second
premise, and no operation waits for the label. One consequence: a number an operation **made** (labelled `valid` as
it was made) which a rule of the program's own hands to a second request is that request's operand like any other,
and is **walked again**, re-deriving the label it already has. Nothing positive tells a made number from one the file
wrote; only a negation or a join could. Is the second walk acceptable (default), should the operations wait for the
label, or should every prefix be labelled as numbers are made so the walk is found done?

Recorded in [`../../open_questions.md` § Q-M1f.410](../../../open_questions.md#q-m1f410--no-join-on-a-number-and-a-result-a-programs-rule-hands-on);
the build is S1f.22.6 (`bf9693dc`), whose record shows it on `decimal-arith/19` (§ *Shown on real programs*, 4).

## 2. What is so today

Both programs run under **before** = `build/review-bin/ein-0a4a97a3` with `EIN_STDLIB=build/wt-rev-0a4/stdlib` (no
check at all: nothing walked, nothing labelled) and **today** = `build/review-bin/ein-head-b5fb8d70`. Answers are
equal under both ([`sum_handed_on.solve.diff`](runs/sum_handed_on.solve.diff),
[`div_named_and_chained.solve.diff`](runs/div_named_and_chained.solve.diff): no difference).

**The smallest** — [`programs/sum_handed_on.ein`](programs/sum_handed_on.ein): 4 + 3, and the rule `again` hands
the sum to `(add ?s (n z 5))`. Every step of today's trace that checks or labels 7, and the rule's step
([`runs/sum_handed_on.today.steps-of-7.txt`](runs/sum_handed_on.today.steps-of-7.txt), from the
[trace](runs/sum_handed_on.today.trace.md)):

<!-- output -->
```
step   19  std.math.positional.turn-end     Derives `…done(…sum(n(z, 4), n(z, 3)), n(z, 7))` …; `std.math.decimal.valid(n(z, 7))`
step   21  again                            Derives `add(n(z, 7), n(z, 5))` — asked: (n z 7) + (n z 5)
step   24  std.math.positional.operand-check  Derives `std.math.decimal.check(n(z, 7), n(z, 7))`
step   26  std.math.decimal.numeral-step    Derives `std.math.decimal.check(n(z, 7), z)`
step   28  std.math.decimal.numeral-valid   Derives `std.math.decimal.valid(n(z, 7))` — (n z 7) is a decimal number
```
7 is labelled at step 19 as it is made, handed on at 21, and walked again at 24–28, step 28 deriving the label it has
(the knowledge base holds it once: [facts](runs/sum_handed_on.today.saturate-facts.txt)). Before: only step 15,
`again`, and no walk ([`runs/sum_handed_on.before.steps-of-7.txt`](runs/sum_handed_on.before.steps-of-7.txt)). Root
26 steps before, 38 today ([trace-steps diff](runs/sum_handed_on.trace-steps.diff)).

**The record's case** — [`programs/div_named_and_chained.ein`](programs/div_named_and_chained.ein), a copy of
`tests/stdlib/math/decimal-arith/19_div_named_and_chained.ein`: `share-out` hands `price × qty` = 1500 to `(div ?total
people)` ([`runs/div_named_and_chained.today.steps-of-1500.txt`](runs/div_named_and_chained.today.steps-of-1500.txt)):
step 104, `turn-end` labels 1500 as the product's last row sum makes it; 112, `share-out`; 118 `operand-check`, 121
and 124 `numeral-step-zero`, 128 and 131 `numeral-step` — five `check` facts of 1500, as the record says — and 134
`numeral-valid`, its label derived again. Root 322 steps before, 342 today
([trace-steps diff](runs/div_named_and_chained.trace-steps.diff)); before, `share-out` at step 99 and no walk
([`runs/div_named_and_chained.before.steps-of-1500.txt`](runs/div_named_and_chained.before.steps-of-1500.txt)).

## 3. The options

- **(a) walked again**, a firing a place, its label already held. **What runs today** (above): one firing a digit and
  one for the label, for each made number a program's rule hands on.
- **(b) the operations wait for the label**, a probe `(valid ?v)` beside the operand (allowed by the static rule: every
  argument bound). Mock: [`runs/rules.option-b.MOCK.txt`](runs/rules.option-b.MOCK.txt) (`add-start` and `div-start`
  with the two probes). It walks the made number again all the same: `operand-check` still asks at the look-up, so
  steps 118–134 are unchanged; what changes is that no walk starts on an unlabelled operand.
- **(c) every prefix of a number labelled as the turn makes it**, so that the ask of a made number finds its walk's
  facts already held: no second walk, and a fact a digit more for every number any walk makes, the intermediate ones
  included. Mock: [`runs/div_named_and_chained.option-c.MOCK.steps-of-1500.txt`](runs/div_named_and_chained.option-c.MOCK.steps-of-1500.txt)
  (the six steps of the second walk marked held already; five prefix labels added over the program — every proper
  prefix of a made number that no fact labels today, computed from the dump). Note the trade: in this program the
  second walk costs six firings and (c) adds five facts; in a program that hands nothing on, (c) only adds.

## 4. The default

**(a)**, built by S1f.22.6 (`bf9693dc`), stated by Q-M1f.404's decision. From the record: *no design under the rule
of the calculator avoids it, short of labelling every prefix of every number a walk makes* (that is (c)); (b) keeps
the rule and walks again all the same; and the ledger: nothing positive tells a made number from a numeral the file
wrote, and only a negation or a join could.

## 5. Files

Programs: `sum_handed_on`, written for this review (claims the answer under both binaries); `div_named_and_chained`, a
copy of `tests/stdlib/math/decimal-arith/19_div_named_and_chained.ein` as at `b5fb8d70`.

Per program and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table*; *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate --dump* whole and *its facts*
(`tools/facts.py`); `ein test`; `ein show hyps`; *kb-cut* `ein render kb` cut to the facts of `check valid written
add sum-of mul div product-of quot-of let` and what they nest, drawn with `fdp` (`tools/kbsub.py`). Each text output
ends with `[exit N]`.

| program | binary | artefacts |
|---|---|---|
| [`sum_handed_on`](programs/sum_handed_on.ein) | before | [read-out](runs/sum_handed_on.before.solve.txt) · [summary](runs/sum_handed_on.before.summary.json) · [events](runs/sum_handed_on.before.events.jsonl) · [events table](runs/sum_handed_on.before.events.md) · [trace](runs/sum_handed_on.before.trace.md) · [dump-answer](runs/sum_handed_on.before.answer/) · [saturate --dump](runs/sum_handed_on.before.saturate-dump.txt) · [its facts](runs/sum_handed_on.before.saturate-facts.txt) · [ein test](runs/sum_handed_on.before.test.txt) · [show hyps](runs/sum_handed_on.before.show-hyps.txt) · [kb-cut.dot](renders/sum_handed_on.before.kb-cut.dot) · [kb-cut.svg](renders/sum_handed_on.before.kb-cut.svg) |
| [`sum_handed_on`](programs/sum_handed_on.ein) | today | [read-out](runs/sum_handed_on.today.solve.txt) · [summary](runs/sum_handed_on.today.summary.json) · [events](runs/sum_handed_on.today.events.jsonl) · [events table](runs/sum_handed_on.today.events.md) · [trace](runs/sum_handed_on.today.trace.md) · [dump-answer](runs/sum_handed_on.today.answer/) · [saturate --dump](runs/sum_handed_on.today.saturate-dump.txt) · [its facts](runs/sum_handed_on.today.saturate-facts.txt) · [ein test](runs/sum_handed_on.today.test.txt) · [show hyps](runs/sum_handed_on.today.show-hyps.txt) · [kb-cut.dot](renders/sum_handed_on.today.kb-cut.dot) · [kb-cut.svg](renders/sum_handed_on.today.kb-cut.svg) |
| [`sum_handed_on`](programs/sum_handed_on.ein) | diff | [saturate-facts.diff](runs/sum_handed_on.saturate-facts.diff) · [solve.diff](runs/sum_handed_on.solve.diff) · [test.diff](runs/sum_handed_on.test.diff) · [trace-steps.diff](runs/sum_handed_on.trace-steps.diff) |
| [`div_named_and_chained`](programs/div_named_and_chained.ein) | before | [read-out](runs/div_named_and_chained.before.solve.txt) · [summary](runs/div_named_and_chained.before.summary.json) · [events](runs/div_named_and_chained.before.events.jsonl) · [events table](runs/div_named_and_chained.before.events.md) · [trace](runs/div_named_and_chained.before.trace.md) · [dump-answer](runs/div_named_and_chained.before.answer/) · [saturate --dump](runs/div_named_and_chained.before.saturate-dump.txt) · [its facts](runs/div_named_and_chained.before.saturate-facts.txt) · [ein test](runs/div_named_and_chained.before.test.txt) · [show hyps](runs/div_named_and_chained.before.show-hyps.txt) · [kb-cut.dot](renders/div_named_and_chained.before.kb-cut.dot) · [kb-cut.svg](renders/div_named_and_chained.before.kb-cut.svg) |
| [`div_named_and_chained`](programs/div_named_and_chained.ein) | today | [read-out](runs/div_named_and_chained.today.solve.txt) · [summary](runs/div_named_and_chained.today.summary.json) · [events](runs/div_named_and_chained.today.events.jsonl) · [events table](runs/div_named_and_chained.today.events.md) · [trace](runs/div_named_and_chained.today.trace.md) · [dump-answer](runs/div_named_and_chained.today.answer/) · [saturate --dump](runs/div_named_and_chained.today.saturate-dump.txt) · [its facts](runs/div_named_and_chained.today.saturate-facts.txt) · [ein test](runs/div_named_and_chained.today.test.txt) · [show hyps](runs/div_named_and_chained.today.show-hyps.txt) · [kb-cut.dot](renders/div_named_and_chained.today.kb-cut.dot) · [kb-cut.svg](renders/div_named_and_chained.today.kb-cut.svg) |
| [`div_named_and_chained`](programs/div_named_and_chained.ein) | diff | [saturate-facts.diff](runs/div_named_and_chained.saturate-facts.diff) · [solve.diff](runs/div_named_and_chained.solve.diff) · [test.diff](runs/div_named_and_chained.test.diff) · [trace-steps.diff](runs/div_named_and_chained.trace-steps.diff) |

Also:

- The steps that check or label the number handed on (`tools/steps_of.py`):
  [`sum_handed_on.before.steps-of-7.txt`](runs/sum_handed_on.before.steps-of-7.txt),
  [`sum_handed_on.today.steps-of-7.txt`](runs/sum_handed_on.today.steps-of-7.txt),
  [`div_named_and_chained.before.steps-of-1500.txt`](runs/div_named_and_chained.before.steps-of-1500.txt),
  [`div_named_and_chained.today.steps-of-1500.txt`](runs/div_named_and_chained.today.steps-of-1500.txt).
- Trace excerpts of every ask, walk and label step (`tools/trace_excerpt.py`):
  [`sum_handed_on.today.trace-check-steps.md`](runs/sum_handed_on.today.trace-check-steps.md),
  [`div_named_and_chained.today.trace-check-steps.md`](runs/div_named_and_chained.today.trace-check-steps.md), and
  the before ones ([sum](runs/sum_handed_on.before.trace-check-steps.md),
  [div](runs/div_named_and_chained.before.trace-check-steps.md)), which hold only `turn-end` (labelling nothing
  then) and the program's rule.
- The mocks: [`rules.option-b.MOCK.txt`](runs/rules.option-b.MOCK.txt),
  [`div_named_and_chained.option-c.MOCK.steps-of-1500.txt`](runs/div_named_and_chained.option-c.MOCK.steps-of-1500.txt).
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/14-Q-M1f.410
build/review-bin/ein-head-b5fb8d70 solve $D/programs/sum_handed_on.ein --trace $D/runs/sum_handed_on.today.trace.md
python3 -I $D/tools/steps_of.py $D/runs/sum_handed_on.today.trace.md 'n(z, 7)' again
EIN_STDLIB=build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 solve $D/programs/sum_handed_on.ein \
  --trace $D/runs/sum_handed_on.before.trace.md
# everything in runs/ and renders/
for p in sum_handed_on div_named_and_chained; do for L in before today; do
  KBRELS="check valid written add sum-of mul div product-of quot-of let" $D/tools/run_all.sh $L $D/programs/$p.ein $D
done; done
$D/tools/diffs.sh $D sum_handed_on div_named_and_chained
for L in before today; do
  python3 -I $D/tools/steps_of.py $D/runs/div_named_and_chained.$L.trace.md 'n(n(n(n(z, 1), 5), 0), 0)' share-out \
    > $D/runs/div_named_and_chained.$L.steps-of-1500.txt
done
```

## 7. Found while building

Nothing that differs from the ledger: the record's five `check` facts of 1500 and its re-derived label reproduce, at
steps 118–134 of today's trace.

## 8. Your call

- **(a)** a made number handed on is walked again — **the default, as built**.
- **(b)** the operations wait for the label (a probe) — and still walk it again.
- **(c)** every prefix labelled as numbers are made — no second walk; a fact a digit more for every number made.
