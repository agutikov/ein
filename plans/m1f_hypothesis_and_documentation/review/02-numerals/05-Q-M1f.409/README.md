# Q-M1f.409 — an integer and a rational: their check and their label

## 1. The question

`std.math.int` writes an integer as `(int P N)`, P less N, each part a natural number; `std.math.rat` writes a
rational as `(rat Z D)`, Z an integer and D a natural number. Once `written` was erased, their *shape* checks (a
digit or a named constant where a part goes) had to move into the operations, as the numerals' did. Which shape
check asks what, and do an integer and a rational get a label of their own, as a numeral gets `(valid N)`? Under the
default, one answer moved outside the corpus: a rational whose numerator is a malformed **natural number**, `(rat (n
z x) (n z 2))`, where an integer goes, was dead before and is alive now — the operation answers it by silence.

Recorded in [`../../open_questions.md` § Q-M1f.409](../../../open_questions.md#q-m1f409--an-integer-and-a-rational-their-check-and-their-label);
the build is S1f.22.6, whose record is read from git
(`git show b5fb8d70^:plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/s1f.22.6_written_erased.md`,
§ *Not done, or uncertain*, first bullet).

## 2. What is so today

Every program was run under **before** = `build/review-bin/ein-0a4a97a3` with `EIN_STDLIB=build/wt-rev-0a4/stdlib`
and **today** = `build/review-bin/ein-head-17085b27`; `*.diff` is `diff -u` before → today.

**The case that moved** — [`programs/d_rat_natural_numerator_malformed.ein`](programs/d_rat_natural_numerator_malformed.ein),
rebuilt from the record: `(rat-add (rat (n z x) (n z 2)) (rat (int (n z 1) z) (n z 2)))`. Its claims are today's
answer.

- **Before: dead**, `unsat core (rat-add (rat (n z x) (n z 2)) …)`, `refuted by
  std.math.decimal.not-decimal-is-false` ([read-out](runs/d_rat_natural_numerator_malformed.before.solve.txt)): the
  loader stated `(written (n z x))` and `numeral-digit` read it
  ([trace excerpt](runs/d_rat_natural_numerator_malformed.before.trace-check-steps.md), steps 2 and 3).
- **Today: alive, R = {∅}, no sum, no flag** ([read-out](runs/d_rat_natural_numerator_malformed.today.solve.txt)).
  The [trace excerpt](runs/d_rat_natural_numerator_malformed.today.trace-check-steps.md) shows why: step 9
  `rat-check-ask` asks `(rat-check (rat (n z x) (n z 2)))`; step 10 `rat-check-parts` asks `int-check(n(z, x))` and
  `check(n(z, 2), n(z, 2))`; the denominator is walked and labelled (steps 11, 12); **no rule reads
  `(int-check (n z x))`** — `int-check-parts` matches only `(int ?p ?n)` — so `(n z x)` is never walked. The
  operation itself computes nothing with a numerator that is no integer (Q-M1f.277), so nothing else touches it.
  Facts ([today](runs/d_rat_natural_numerator_malformed.today.saturate-facts.txt)):

  <!-- output -->
  ```
    (std.math.decimal.check (n z 2) (n z 2) :rule std.math.rat.rat-check-parts)
    (std.math.decimal.check (n z 2) z :rule std.math.decimal.numeral-step)
    (std.math.decimal.valid z :rule std.math.decimal.decimal)
    (std.math.decimal.valid (n z 2) :rule std.math.decimal.numeral-valid)
    (std.math.int.int-check (n z x) :rule std.math.rat.rat-check-parts)
    (std.math.rat.rat-check (rat (n z x) (n z 2)) :rule std.math.rat.rat-check-ask)
  ```
  Diffs: [read-out](runs/d_rat_natural_numerator_malformed.solve.diff),
  [facts](runs/d_rat_natural_numerator_malformed.saturate-facts.diff),
  [trace steps](runs/d_rat_natural_numerator_malformed.trace-steps.diff),
  [ein test](runs/d_rat_natural_numerator_malformed.test.diff) (before 3 FAILED, exit 1; today 3 held). KB views
  cut to the check's relations: [before](renders/d_rat_natural_numerator_malformed.before.kb-cut.svg),
  [today](renders/d_rat_natural_numerator_malformed.today.kb-cut.svg).

**Its twin, unmoved** — [`programs/d_int_natural_malformed.ein`](programs/d_int_natural_malformed.ein),
`(int-add (n z x) (int (n z 1) z))`: dead under both, `refuted by std.math.decimal.not-decimal-is-false`
([before](runs/d_int_natural_malformed.before.solve.txt), [today](runs/d_int_natural_malformed.today.solve.txt)).
Today `std.math.decimal-arith`'s look-up finds the natural number and `operand-check` asks its check
([facts](runs/d_int_natural_malformed.today.saturate-facts.txt): `(check (n z x) (n z x) :rule
std.math.positional.operand-check)`). So where an integer goes a malformed natural is flagged, and where a rational's
numerator goes it is not.

**The design as built, on well-formed numbers** — [`programs/a_int_and_rat_well_formed.ein`](programs/a_int_and_rat_well_formed.ein),
7 + (−3) and 1/2 + 1/3. Answers equal under both binaries ([diff](runs/a_int_and_rat_well_formed.solve.diff): no
difference). Today ([facts](runs/a_int_and_rat_well_formed.today.saturate-facts.txt),
[trace excerpt](runs/a_int_and_rat_well_formed.today.trace-check-steps.md)):

<!-- output -->
```
  (std.math.int.int-check (int (n z 7) z) :rule std.math.int.int-operand-check)
  (std.math.int.int-check (int (n z 1) z) :rule std.math.rat.rat-check-parts)
  (std.math.int.int-check (int z (n z 3)) :rule std.math.int.int-operand-check)
  (std.math.rat.rat-check (rat (int (n z 1) z) (n z 2)) :rule std.math.rat.rat-check-ask)
  (std.math.rat.rat-check (rat (int (n z 1) z) (n z 3)) :rule std.math.rat.rat-check-ask)
  (std.math.decimal.check (n z 7) (n z 7) :rule std.math.int.int-check-parts)
  (std.math.decimal.check z z :rule std.math.int.int-check-parts)
  …
  (std.math.decimal.valid (n z 7) :rule std.math.decimal.numeral-valid)
  (std.math.decimal.valid (n z 4) :rule std.math.positional.turn-end)
```
The integer and the rational are asked (`int-check`, `rat-check`), their numerals are walked and labelled `valid`,
and there is **no `int-valid` or `rat-valid`**: the label is the numerals'. Note `(check z z)`: a part `z` is asked
too, which re-derives `(valid z)` (finding L-101, `../29-L-101/`).

**A shape fault** — [`programs/c_int_digit_as_a_part.ein`](programs/c_int_digit_as_a_part.ein), `(int-add (int 7 z)
(int (n z 1) z))`: dead under both, by `int-p-digit` — before it read `(written (int 7 z))`, today `(int-check (int
7 z))` ([facts diff](runs/c_int_digit_as_a_part.saturate-facts.diff)); `unsat core (int-add (int 7 z) (int (n z 1)
z))`, `(std.math.digits.nz 7)`, `refuted by std.math.int.not-integer-is-false`
([today](runs/c_int_digit_as_a_part.today.solve.txt)).

## 3. The options

- **(a) `(int-check I)` and `(rat-check Q)`, asked where an operand's look-up finds one; no label of their own.**
  What runs today (above). Its moved case: `d_rat_natural_numerator_malformed` alive.
- **(b) a label of their own, `(int-valid I)`**, by a walk that carries the integer through both parts in turn
  (joining the parts' labels would join two facts on a number). Every part is then walked twice, once as a numeral
  and once as an integer. Mock: [`runs/a_int_and_rat_well_formed.option-b.MOCK.saturate-facts.txt`](runs/a_int_and_rat_well_formed.option-b.MOCK.saturate-facts.txt)
  — today's facts with each integer's and rational's second walk and its label added (`int-walk`, `rat-walk` and
  the rule names made up for the mock). No answer changes.
- **(c) the shapes left unchecked, only the numerals**: `(int 7 z)` answered by silence.
  `c_int_digit_as_a_part` would be alive: mocks
  [`runs/c_int_digit_as_a_part.option-c.MOCK.saturate-facts.txt`](runs/c_int_digit_as_a_part.option-c.MOCK.saturate-facts.txt)
  (the flag and `(false)` deleted) and
  [`runs/c_int_digit_as_a_part.option-c.MOCK.solve.txt`](runs/c_int_digit_as_a_part.option-c.MOCK.solve.txt)
  (the alive read-out, the claim FAILED).
- **(d) as (a), and `rat-check-parts` asks `std.math.decimal`'s check of a numerator that is a natural number.**
  `d_rat_natural_numerator_malformed` dead again, as before: mocks
  [`runs/d_rat_natural_numerator_malformed.option-d.MOCK.saturate-facts.txt`](runs/d_rat_natural_numerator_malformed.option-d.MOCK.saturate-facts.txt)
  (the ask of `(n z x)`, by a rule named `rat-check-natural` there, its flag and `(false)` added) and
  [`runs/d_rat_natural_numerator_malformed.option-d.MOCK.solve.txt`](runs/d_rat_natural_numerator_malformed.option-d.MOCK.solve.txt)
  (the dead read-out, its lines the before binary's). The rational's numerator would then be flagged as an
  integer's operand already is (`d_int_natural_malformed`).

## 4. The default

**(a)**, built by S1f.22.6 (`bf9693dc`), stated by Q-M1f.404's decision. From the ledger: the shape rules flag a
digit or a named constant where a part goes, as before; the label is the numerals', so no second walk (b's cost);
(c) would answer a shape fault by silence; and the (d) case is one no corpus program writes, the operation
computing nothing with a numerator that is no integer (Q-M1f.277). Where an integer goes, a natural number is
still flagged.

## 5. Files

Programs: all four are written for this review, each claiming today's answer.

Per program and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table* (markdown, `tools/jsonl2md.py`); *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate
--dump* the whole dump and *its facts* (the facts that bear on the check, `tools/facts.py`); `ein test`; `ein show
hyps`; *kb-cut* `ein render kb` cut to the facts of `check valid not-decimal written int-check rat-check not-integer
not-rational int-add rat-add int-sum-of rat-sum-of` and what they nest (`tools/kbsub.py`, every line kept the
binary's own), drawn with `fdp`. The whole KB view is not kept (mostly the digit tables, some 10 000 lines). Each
text output ends with the command's exit code, `[exit N]`.

*Today*'s kb-cut is drawn as the graph has it: a fact an octagon with no name in it, its arrow `1` at its relation's
box and `2`, `3`, … at its arguments, what it rests on (`by RULE`) beside it, and a fact under a fact's arrow that the
knowledge base does not hold in the frame of what points at it (`add ctx`); *before*'s is the earlier binary's
drawing, labelled octagons and labelled arrows. `tools/kbsub.py` reads both forms.

| program | binary | artefacts |
|---|---|---|
| [`d_rat_natural_numerator_malformed`](programs/d_rat_natural_numerator_malformed.ein) | before | [read-out](runs/d_rat_natural_numerator_malformed.before.solve.txt) · [summary](runs/d_rat_natural_numerator_malformed.before.summary.json) · [events](runs/d_rat_natural_numerator_malformed.before.events.jsonl) · [events table](runs/d_rat_natural_numerator_malformed.before.events.md) · [trace](runs/d_rat_natural_numerator_malformed.before.trace.md) · [dump-answer](runs/d_rat_natural_numerator_malformed.before.answer/) · [saturate --dump](runs/d_rat_natural_numerator_malformed.before.saturate-dump.txt) · [its facts](runs/d_rat_natural_numerator_malformed.before.saturate-facts.txt) · [ein test](runs/d_rat_natural_numerator_malformed.before.test.txt) · [show hyps](runs/d_rat_natural_numerator_malformed.before.show-hyps.txt) · [kb-cut.dot](renders/d_rat_natural_numerator_malformed.before.kb-cut.dot) · [kb-cut.svg](renders/d_rat_natural_numerator_malformed.before.kb-cut.svg) |
| [`d_rat_natural_numerator_malformed`](programs/d_rat_natural_numerator_malformed.ein) | today | [read-out](runs/d_rat_natural_numerator_malformed.today.solve.txt) · [summary](runs/d_rat_natural_numerator_malformed.today.summary.json) · [events](runs/d_rat_natural_numerator_malformed.today.events.jsonl) · [events table](runs/d_rat_natural_numerator_malformed.today.events.md) · [trace](runs/d_rat_natural_numerator_malformed.today.trace.md) · [dump-answer](runs/d_rat_natural_numerator_malformed.today.answer/) · [saturate --dump](runs/d_rat_natural_numerator_malformed.today.saturate-dump.txt) · [its facts](runs/d_rat_natural_numerator_malformed.today.saturate-facts.txt) · [ein test](runs/d_rat_natural_numerator_malformed.today.test.txt) · [show hyps](runs/d_rat_natural_numerator_malformed.today.show-hyps.txt) · [kb-cut.dot](renders/d_rat_natural_numerator_malformed.today.kb-cut.dot) · [kb-cut.svg](renders/d_rat_natural_numerator_malformed.today.kb-cut.svg) |
| [`d_rat_natural_numerator_malformed`](programs/d_rat_natural_numerator_malformed.ein) | diff | [saturate-facts.diff](runs/d_rat_natural_numerator_malformed.saturate-facts.diff) · [solve.diff](runs/d_rat_natural_numerator_malformed.solve.diff) · [test.diff](runs/d_rat_natural_numerator_malformed.test.diff) · [trace-steps.diff](runs/d_rat_natural_numerator_malformed.trace-steps.diff) |
| [`d_int_natural_malformed`](programs/d_int_natural_malformed.ein) | before | [read-out](runs/d_int_natural_malformed.before.solve.txt) · [summary](runs/d_int_natural_malformed.before.summary.json) · [events](runs/d_int_natural_malformed.before.events.jsonl) · [events table](runs/d_int_natural_malformed.before.events.md) · [trace](runs/d_int_natural_malformed.before.trace.md) · [dump-answer](runs/d_int_natural_malformed.before.answer/) · [saturate --dump](runs/d_int_natural_malformed.before.saturate-dump.txt) · [its facts](runs/d_int_natural_malformed.before.saturate-facts.txt) · [ein test](runs/d_int_natural_malformed.before.test.txt) · [show hyps](runs/d_int_natural_malformed.before.show-hyps.txt) · [kb-cut.dot](renders/d_int_natural_malformed.before.kb-cut.dot) · [kb-cut.svg](renders/d_int_natural_malformed.before.kb-cut.svg) |
| [`d_int_natural_malformed`](programs/d_int_natural_malformed.ein) | today | [read-out](runs/d_int_natural_malformed.today.solve.txt) · [summary](runs/d_int_natural_malformed.today.summary.json) · [events](runs/d_int_natural_malformed.today.events.jsonl) · [events table](runs/d_int_natural_malformed.today.events.md) · [trace](runs/d_int_natural_malformed.today.trace.md) · [dump-answer](runs/d_int_natural_malformed.today.answer/) · [saturate --dump](runs/d_int_natural_malformed.today.saturate-dump.txt) · [its facts](runs/d_int_natural_malformed.today.saturate-facts.txt) · [ein test](runs/d_int_natural_malformed.today.test.txt) · [show hyps](runs/d_int_natural_malformed.today.show-hyps.txt) · [kb-cut.dot](renders/d_int_natural_malformed.today.kb-cut.dot) · [kb-cut.svg](renders/d_int_natural_malformed.today.kb-cut.svg) |
| [`d_int_natural_malformed`](programs/d_int_natural_malformed.ein) | diff | [saturate-facts.diff](runs/d_int_natural_malformed.saturate-facts.diff) · [solve.diff](runs/d_int_natural_malformed.solve.diff) · [test.diff](runs/d_int_natural_malformed.test.diff) · [trace-steps.diff](runs/d_int_natural_malformed.trace-steps.diff) |
| [`a_int_and_rat_well_formed`](programs/a_int_and_rat_well_formed.ein) | before | [read-out](runs/a_int_and_rat_well_formed.before.solve.txt) · [summary](runs/a_int_and_rat_well_formed.before.summary.json) · [events](runs/a_int_and_rat_well_formed.before.events.jsonl) · [events table](runs/a_int_and_rat_well_formed.before.events.md) · [trace](runs/a_int_and_rat_well_formed.before.trace.md) · [dump-answer](runs/a_int_and_rat_well_formed.before.answer/) · [saturate --dump](runs/a_int_and_rat_well_formed.before.saturate-dump.txt) · [its facts](runs/a_int_and_rat_well_formed.before.saturate-facts.txt) · [ein test](runs/a_int_and_rat_well_formed.before.test.txt) · [show hyps](runs/a_int_and_rat_well_formed.before.show-hyps.txt) · [kb-cut.dot](renders/a_int_and_rat_well_formed.before.kb-cut.dot) · [kb-cut.svg](renders/a_int_and_rat_well_formed.before.kb-cut.svg) |
| [`a_int_and_rat_well_formed`](programs/a_int_and_rat_well_formed.ein) | today | [read-out](runs/a_int_and_rat_well_formed.today.solve.txt) · [summary](runs/a_int_and_rat_well_formed.today.summary.json) · [events](runs/a_int_and_rat_well_formed.today.events.jsonl) · [events table](runs/a_int_and_rat_well_formed.today.events.md) · [trace](runs/a_int_and_rat_well_formed.today.trace.md) · [dump-answer](runs/a_int_and_rat_well_formed.today.answer/) · [saturate --dump](runs/a_int_and_rat_well_formed.today.saturate-dump.txt) · [its facts](runs/a_int_and_rat_well_formed.today.saturate-facts.txt) · [ein test](runs/a_int_and_rat_well_formed.today.test.txt) · [show hyps](runs/a_int_and_rat_well_formed.today.show-hyps.txt) · [kb-cut.dot](renders/a_int_and_rat_well_formed.today.kb-cut.dot) · [kb-cut.svg](renders/a_int_and_rat_well_formed.today.kb-cut.svg) |
| [`a_int_and_rat_well_formed`](programs/a_int_and_rat_well_formed.ein) | diff | [saturate-facts.diff](runs/a_int_and_rat_well_formed.saturate-facts.diff) · [solve.diff](runs/a_int_and_rat_well_formed.solve.diff) · [test.diff](runs/a_int_and_rat_well_formed.test.diff) · [trace-steps.diff](runs/a_int_and_rat_well_formed.trace-steps.diff) |
| [`c_int_digit_as_a_part`](programs/c_int_digit_as_a_part.ein) | before | [read-out](runs/c_int_digit_as_a_part.before.solve.txt) · [summary](runs/c_int_digit_as_a_part.before.summary.json) · [events](runs/c_int_digit_as_a_part.before.events.jsonl) · [events table](runs/c_int_digit_as_a_part.before.events.md) · [trace](runs/c_int_digit_as_a_part.before.trace.md) · [dump-answer](runs/c_int_digit_as_a_part.before.answer/) · [saturate --dump](runs/c_int_digit_as_a_part.before.saturate-dump.txt) · [its facts](runs/c_int_digit_as_a_part.before.saturate-facts.txt) · [ein test](runs/c_int_digit_as_a_part.before.test.txt) · [show hyps](runs/c_int_digit_as_a_part.before.show-hyps.txt) · [kb-cut.dot](renders/c_int_digit_as_a_part.before.kb-cut.dot) · [kb-cut.svg](renders/c_int_digit_as_a_part.before.kb-cut.svg) |
| [`c_int_digit_as_a_part`](programs/c_int_digit_as_a_part.ein) | today | [read-out](runs/c_int_digit_as_a_part.today.solve.txt) · [summary](runs/c_int_digit_as_a_part.today.summary.json) · [events](runs/c_int_digit_as_a_part.today.events.jsonl) · [events table](runs/c_int_digit_as_a_part.today.events.md) · [trace](runs/c_int_digit_as_a_part.today.trace.md) · [dump-answer](runs/c_int_digit_as_a_part.today.answer/) · [saturate --dump](runs/c_int_digit_as_a_part.today.saturate-dump.txt) · [its facts](runs/c_int_digit_as_a_part.today.saturate-facts.txt) · [ein test](runs/c_int_digit_as_a_part.today.test.txt) · [show hyps](runs/c_int_digit_as_a_part.today.show-hyps.txt) · [kb-cut.dot](renders/c_int_digit_as_a_part.today.kb-cut.dot) · [kb-cut.svg](renders/c_int_digit_as_a_part.today.kb-cut.svg) |
| [`c_int_digit_as_a_part`](programs/c_int_digit_as_a_part.ein) | diff | [saturate-facts.diff](runs/c_int_digit_as_a_part.saturate-facts.diff) · [solve.diff](runs/c_int_digit_as_a_part.solve.diff) · [test.diff](runs/c_int_digit_as_a_part.test.diff) · [trace-steps.diff](runs/c_int_digit_as_a_part.trace-steps.diff) |

Also:

- Trace excerpts (the steps that ask, walk, label and flag; `tools/trace_excerpt.py`):
  [`d_rat_natural_numerator_malformed.before.trace-check-steps.md`](runs/d_rat_natural_numerator_malformed.before.trace-check-steps.md),
  [`d_rat_natural_numerator_malformed.today.trace-check-steps.md`](runs/d_rat_natural_numerator_malformed.today.trace-check-steps.md),
  [`a_int_and_rat_well_formed.today.trace-check-steps.md`](runs/a_int_and_rat_well_formed.today.trace-check-steps.md).
- The mocks (hand-edited, each says at its top what was edited):
  [`a_int_and_rat_well_formed.option-b.MOCK.saturate-facts.txt`](runs/a_int_and_rat_well_formed.option-b.MOCK.saturate-facts.txt),
  [`c_int_digit_as_a_part.option-c.MOCK.saturate-facts.txt`](runs/c_int_digit_as_a_part.option-c.MOCK.saturate-facts.txt),
  [`c_int_digit_as_a_part.option-c.MOCK.solve.txt`](runs/c_int_digit_as_a_part.option-c.MOCK.solve.txt),
  [`d_rat_natural_numerator_malformed.option-d.MOCK.saturate-facts.txt`](runs/d_rat_natural_numerator_malformed.option-d.MOCK.saturate-facts.txt),
  [`d_rat_natural_numerator_malformed.option-d.MOCK.solve.txt`](runs/d_rat_natural_numerator_malformed.option-d.MOCK.solve.txt).
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/05-Q-M1f.409
build/review-bin/ein-head-17085b27 solve $D/programs/d_rat_natural_numerator_malformed.ein
EIN_STDLIB=build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 solve $D/programs/d_rat_natural_numerator_malformed.ein
build/review-bin/ein-head-17085b27 saturate $D/programs/d_rat_natural_numerator_malformed.ein --dump
# everything in runs/ and renders/ (the mocks are hand-made and are not re-made)
P="d_rat_natural_numerator_malformed d_int_natural_malformed a_int_and_rat_well_formed c_int_digit_as_a_part"
for p in $P; do for L in before today; do
  KBRELS="check valid not-decimal written int-check rat-check not-integer not-rational int-add rat-add int-sum-of rat-sum-of" \
    $D/tools/run_all.sh $L $D/programs/$p.ein $D
done; done
$D/tools/diffs.sh $D $P
python3 -I $D/tools/trace_excerpt.py $D/runs/d_rat_natural_numerator_malformed.today.trace.md \
  $D/runs/d_rat_natural_numerator_malformed.today.trace-check-steps.md \
  rat-check-ask rat-check-parts int-operand-check int-check-parts numeral-step numeral-valid numeral-digit
```

## 7. Found while building

- The record's case reproduces as stated (`states (k_s) 1` today, `0` before), and so does its twin
  (`(int-add (n z x) I)` dead under both).
- Today the moved case leaves `(std.math.int.int-check (n z x))` in the knowledge base, a fact no rule reads: the
  third point of finding L-101 (`../29-L-101/`), seen here on the very program the default moved.
- Only the first operand's rational is checked in the moved case: the second, `(rat (int (n z 1) z) (n z 2))`, is
  never looked up, since the look-up of the second operand waits on the first being an integer over a number.
- **Re-taken with `ein-head-17085b27`** (the first look ran `ein-head-b5fb8d70`; the standard library and
  `tests/stdlib/` are the same in both trees, `git diff --stat b5fb8d70 17085b27 -- stdlib tests/stdlib` is empty):
  every read-out, summary, event stream, dump, `ein test` and `ein show hyps` output is the same, wall-clock aside,
  and so is every trace's text and every excerpt of it; what moved is the drawings, each trace's `dot` blocks and
  today's KB cuts, now in the canonical form. The before binary's outputs re-run the same.

## 8. Your call

- **(a)** `int-check` and `rat-check` where a look-up finds one, no label of their own — **the default, as built**.
- **(b)** a label of their own, `(int-valid I)`, by a second walk through the parts.
- **(c)** shapes unchecked, only the numerals — `(int 7 z)` answered by silence.
- **(d)** as (a), and a rational's natural numerator checked as a numeral — the moved case dead again.
